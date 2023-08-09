#[cfg(test)]
mod integration_tests {
    use anyhow::{anyhow, Context, Result};
    use futures::{
        channel::{mpsc, oneshot},
        future,
        stream::{self, FuturesUnordered, SplitSink},
        task::Poll,
        FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt,
    };
    use hyper::{
        body::Bytes, http::Error, service, Body, Client, Method, Request, Response, Server,
        StatusCode,
    };
    use redis_protocol::resp3::types::Frame;
    use sha2::{Digest, Sha256};
    use spin_loader::local::{config::RawModuleSource, raw_manifest_from_file};
    use spin_trigger_wasi_messaging::RedisCodec;
    use std::{
        collections::HashMap,
        ffi::OsStr,
        iter,
        net::{Ipv4Addr, SocketAddrV4, TcpListener},
        ops::Deref,
        path::Path,
        pin::Pin,
        process::{self, Child, Command, Output},
        str,
        sync::Arc,
        time::Duration,
    };
    use tempfile::tempdir;
    use tokio::{net::TcpStream, sync::Mutex as AsyncMutex, task, time::sleep};
    use tokio_util::codec::Framed;

    const TIMER_TRIGGER_INTEGRATION_TEST: &str = "examples/spin-timer/app-example";
    const TIMER_TRIGGER_DIRECTORY: &str = "examples/spin-timer";

    const RUST_HTTP_INTEGRATION_TEST: &str = "tests/http/simple-spin-rust";

    const DEFAULT_MANIFEST_LOCATION: &str = "spin.toml";

    const SPIN_BINARY: &str = "./target/debug/spin";

    #[cfg(feature = "outbound-redis-tests")]
    mod outbound_redis_tests {
        use super::*;

        const RUST_OUTBOUND_REDIS_INTEGRATION_TEST: &str =
            "tests/outbound-redis/http-rust-outbound-redis";

        #[tokio::test]
        async fn test_outbound_redis_rust_local() -> Result<()> {
            let s = SpinTestController::with_manifest(
                &format!(
                    "{}/{}",
                    RUST_OUTBOUND_REDIS_INTEGRATION_TEST, DEFAULT_MANIFEST_LOCATION
                ),
                &[],
                &[],
                None,
            )
            .await?;

            assert_status(&s, "/test", 204).await?;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_simple_rust_local() -> Result<()> {
        let s = SpinTestController::with_manifest(
            &format!(
                "{}/{}",
                RUST_HTTP_INTEGRATION_TEST, DEFAULT_MANIFEST_LOCATION
            ),
            &[],
            &[],
            None,
        )
        .await?;

        assert_status(&s, "/test/hello", 200).await?;
        assert_status(&s, "/test/hello/wildcards/should/be/handled", 200).await?;
        assert_status(&s, "/thisshouldfail", 404).await?;
        assert_status(&s, "/test/hello/test-placement", 200).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_timer_trigger() -> Result<()> {
        use std::fs;

        let trigger_dir = Path::new(TIMER_TRIGGER_DIRECTORY);

        // Conventionally, we would do all Cargo builds of test code in build.rs, but this one can take a lot
        // longer than the tiny tests we normally build there (and it's pointless if the user just wants to build
        // Spin without running any tests) so we do it here instead.  Subsequent builds after the first one should
        // be very fast.
        assert!(Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(trigger_dir)
            .status()?
            .success());

        // Create a test plugin store so we don't modify the user's real one.
        let plugin_store_dir = Path::new(concat!(env!("OUT_DIR"), "/plugin-store"));
        let plugins_dir = plugin_store_dir.join("spin/plugins");

        let plugin_dir = plugins_dir.join("trigger-timer");
        fs::create_dir_all(&plugin_dir)?;
        fs::copy(
            trigger_dir.join("target/release/trigger-timer"),
            plugin_dir.join("trigger-timer"),
        )?;

        let manifests_dir = plugins_dir.join("manifests");
        fs::create_dir_all(&manifests_dir)?;
        // Note that the hash and path in the manifest aren't accurate, but they won't be used anyway for this
        // test.  We just need something that parses without throwing errors here.
        fs::copy(
            Path::new(TIMER_TRIGGER_DIRECTORY).join("trigger-timer.json"),
            manifests_dir.join("trigger-timer.json"),
        )?;

        assert!(Command::new(get_process(SPIN_BINARY))
            .args([
                "up",
                "--file",
                &format!("{TIMER_TRIGGER_INTEGRATION_TEST}/{DEFAULT_MANIFEST_LOCATION}"),
                "--test",
            ])
            .env("TEST_PLUGINS_DIRECTORY", plugin_store_dir)
            .status()?
            .success());

        Ok(())
    }

    #[cfg(feature = "config-provider-tests")]
    mod config_provider_tests {
        use super::*;

        const RUST_HTTP_VAULT_CONFIG_TEST: &str = "tests/http/vault-config-test";
        const VAULT_BINARY: &str = "vault";
        const VAULT_ROOT_TOKEN: &str = "root";

        #[tokio::test]
        async fn test_vault_config_provider() -> Result<()> {
            let vault = VaultTestController::new().await?;
            let http_client = reqwest::Client::new();
            let data = r#"
{
    "data": {
        "value": "test_password"
    }
}
"#;
            let body_map: HashMap<String, HashMap<String, String>> = serde_json::from_str(data)?;
            let status = http_client
                .post(format!("{}/v1/secret/data/password", &vault.url))
                .header("X-Vault-Token", VAULT_ROOT_TOKEN)
                .json(&body_map)
                .send()
                .await?
                .status();
            assert_eq!(status, 200);

            let s = SpinTestController::with_manifest(
                &format!(
                    "{}/{}",
                    RUST_HTTP_VAULT_CONFIG_TEST, DEFAULT_MANIFEST_LOCATION
                ),
                &[
                    "--runtime-config-file",
                    &format!("{}/{}", RUST_HTTP_VAULT_CONFIG_TEST, "runtime_config.toml"),
                ],
                &[],
                None,
            )
            .await?;

            assert_status(&s, "/", 200).await?;

            Ok(())
        }

        /// Controller for running Vault.
        pub struct VaultTestController {
            pub url: String,
            vault_handle: Child,
        }

        impl VaultTestController {
            pub async fn new() -> Result<VaultTestController> {
                let address = "127.0.0.1:8200";
                let url = format!("http://{}", address);

                let mut vault_handle = Command::new(get_process(VAULT_BINARY))
                    .args(["server", "-dev", "-dev-root-token-id", VAULT_ROOT_TOKEN])
                    .spawn()
                    .with_context(|| "executing vault")?;

                wait_vault(&url, &mut vault_handle, VAULT_BINARY).await?;

                Ok(Self { url, vault_handle })
            }
        }

        impl Drop for VaultTestController {
            fn drop(&mut self) {
                let _ = self.vault_handle.kill();
            }
        }

        async fn wait_vault(url: &str, process: &mut Child, target: &str) -> Result<()> {
            println!("vault url is {} and process is {:?}", url, process);
            let mut wait_count = 0;
            loop {
                if wait_count >= 120 {
                    panic!(
                        "Ran out of retries waiting for {} to start on URL {}",
                        target, url
                    );
                }

                if let Ok(Some(_)) = process.try_wait() {
                    panic!(
                        "Process exited before starting to serve {} to start on URL {}",
                        target, url
                    );
                }

                let client = reqwest::Client::new();
                if let Ok(rsp) = client
                    .get(format!("{url}/v1/sys/health"))
                    .header("X-Vault-Token", VAULT_ROOT_TOKEN)
                    .send()
                    .await
                {
                    if rsp.status().is_success() {
                        break;
                    }
                }

                wait_count += 1;
                sleep(Duration::from_secs(1)).await;
            }

            Ok(())
        }
    }

    async fn assert_status(
        s: &SpinTestController,
        absolute_uri: &str,
        expected: u16,
    ) -> Result<()> {
        let res = req(s, absolute_uri).await?;
        let status = res.status();
        let body = hyper::body::to_bytes(res.into_body())
            .await
            .expect("read body");
        assert_eq!(status, expected, "{}", String::from_utf8_lossy(&body));

        Ok(())
    }

    async fn req(s: &SpinTestController, absolute_uri: &str) -> Result<Response<Body>> {
        let c = Client::new();
        let url = format!("http://{}{}", s.url, absolute_uri)
            .parse()
            .with_context(|| "cannot parse URL")?;
        Ok(c.get(url).await?)
    }

    /// Controller for running Spin.
    pub struct SpinTestController {
        pub url: String,
        spin_handle: Child,
    }

    impl SpinTestController {
        pub async fn with_manifest(
            manifest_path: &str,
            spin_args: &[&str],
            spin_app_env: &[&str],
            bindle_url: Option<&str>,
        ) -> Result<SpinTestController> {
            // start Spin using the given application manifest and wait for the HTTP server to be available.
            let url = format!("127.0.0.1:{}", get_random_port()?);
            let mut args = vec!["up", "--file", manifest_path, "--listen", &url];
            args.extend(spin_args);
            if let Some(b) = bindle_url {
                args.push("--bindle-server");
                args.push(b);
            }
            for v in spin_app_env {
                args.push("--env");
                args.push(v);
            }

            let mut spin_handle = Command::new(get_process(SPIN_BINARY))
                .args(args)
                .env(
                    "RUST_LOG",
                    "spin=trace,spin_loader=trace,spin_core=trace,spin_http=trace",
                )
                .spawn()
                .with_context(|| "executing Spin")?;

            // ensure the server is accepting requests before continuing.
            wait_tcp(&url, &mut spin_handle, SPIN_BINARY).await?;

            Ok(SpinTestController { url, spin_handle })
        }
    }

    impl Drop for SpinTestController {
        fn drop(&mut self) {
            #[cfg(windows)]
            let _ = self.spin_handle.kill();
            #[cfg(not(windows))]
            {
                let pid = nix::unistd::Pid::from_raw(self.spin_handle.id() as i32);
                let _ = nix::sys::signal::kill(pid, nix::sys::signal::SIGTERM);
            }
        }
    }

    fn run<S: Into<String> + AsRef<OsStr>>(
        args: Vec<S>,
        dir: Option<S>,
        envs: Option<HashMap<&str, &str>>,
    ) -> Result<Output> {
        let mut cmd = Command::new(get_os_process());
        cmd.stdout(process::Stdio::piped());
        cmd.stderr(process::Stdio::piped());

        if let Some(dir) = dir {
            cmd.current_dir(dir.into());
        };

        cmd.arg("-c");
        cmd.arg(
            args.into_iter()
                .map(Into::into)
                .collect::<Vec<String>>()
                .join(" "),
        );

        cmd.env("RUST_LOG", "spin_cli=warn");
        if let Some(envs) = envs {
            for (k, v) in envs {
                cmd.env(k, v);
            }
        }

        let output = cmd.output()?;
        println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        let code = output.status.code().expect("should have status code");
        if code != 0 {
            panic!("command `{:?}` exited with code {}", cmd, code);
        }

        Ok(output)
    }

    fn get_process(binary: &str) -> String {
        if cfg!(target_os = "windows") {
            format!("{}.exe", binary)
        } else {
            binary.to_string()
        }
    }

    fn get_os_process() -> String {
        if cfg!(target_os = "windows") {
            String::from("powershell.exe")
        } else {
            String::from("bash")
        }
    }

    fn get_random_port() -> Result<u16> {
        Ok(
            TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?
                .local_addr()?
                .port(),
        )
    }

    async fn wait_tcp(url: &str, process: &mut Child, target: &str) -> Result<()> {
        let mut wait_count = 0;
        loop {
            if wait_count >= 240 {
                panic!(
                    "Ran out of retries waiting for {} to start on URL {}",
                    target, url
                );
            }

            if let Ok(Some(_)) = process.try_wait() {
                panic!(
                    "Process exited before starting to serve {} to start on URL {}",
                    target, url
                );
            }

            match TcpStream::connect(&url).await {
                Ok(_) => break,
                Err(e) => {
                    println!("connect {} error {}, retry {}", &url, e, wait_count);
                    wait_count += 1;
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Ok(())
    }

    /// Builds app in `dir` and verifies the build succeeded. Expects manifest
    /// in `spin.toml` inside `dir`.
    async fn do_test_build_command(dir: impl AsRef<Path>) -> Result<()> {
        let manifest_file = dir.as_ref().join("spin.toml");
        let manifest = raw_manifest_from_file(&manifest_file).await?.into_v1();

        let mut sources = vec![];
        for component_manifest in manifest.components.iter() {
            if let RawModuleSource::FileReference(file) = &component_manifest.source {
                sources.push(dir.as_ref().join(file));
            } else {
                panic!(
                    "{}.{}: source is not a file reference",
                    manifest.info.name, component_manifest.id
                )
            }
        }

        // Delete build output so that later it can be assumed: if the output
        // exists, it is because `spin build` succeeded.
        for source in sources.iter() {
            if source.exists() {
                std::fs::remove_file(source)?
            }
        }

        run(
            vec![
                SPIN_BINARY,
                "build",
                "--file",
                manifest_file.to_str().unwrap(),
            ],
            None,
            None,
        )?;

        let mut missing_sources_count = 0;
        for (i, source) in sources.iter().enumerate() {
            if source.exists() {
                std::fs::remove_file(source)?;
            } else {
                missing_sources_count += 1;
                println!(
                    "{}.{} source file was not generated by build",
                    manifest.info.name, manifest.components[i].id
                );
            }
        }
        assert_eq!(missing_sources_count, 0);

        Ok(())
    }

    #[test]
    fn spin_up_gives_help_on_new_app() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir = temp_dir.path();
        let manifest_file = dir.join("spin.toml");

        // We still don't see full help if there are no components.
        let toml_text = r#"spin_version = "1"
name = "unbuilt"
trigger = { type = "http", base = "/" }
version = "0.1.0"
[[component]]
id = "unbuilt"
source = "DOES-NOT-EXIST.wasm"
[component.trigger]
route = "/..."
"#;

        std::fs::write(&manifest_file, toml_text)?;

        let up_help_args = vec![
            SPIN_BINARY,
            "up",
            "--file",
            manifest_file.to_str().unwrap(),
            "--help",
        ];

        let output = run(up_help_args, None, None)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("--quiet"));
        assert!(stdout.contains("--listen"));

        Ok(())
    }

    // TODO: Test on Windows
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_spin_plugin_install_command() -> Result<()> {
        // Create a temporary directory for plugin source and manifests
        let temp_dir = tempdir()?;
        let dir = temp_dir.path();
        let installed_plugins_dir = dir.join("tmp");

        // Ensure that spin installs the plugins into the temporary directory
        let mut env_map: HashMap<&str, &str> = HashMap::new();
        env_map.insert(
            "TEST_PLUGINS_DIRECTORY",
            installed_plugins_dir.to_str().unwrap(),
        );

        let path_to_test_dir = std::env::current_dir()?;
        let file_url = format!(
            "file:{}/tests/plugin/example.tar.gz",
            path_to_test_dir.to_str().unwrap()
        );
        let mut plugin_manifest_json = serde_json::json!(
        {
            "name": "example",
            "description": "A description of the plugin.",
            "homepage": "www.example.com",
            "version": "0.2.0",
            "spinCompatibility": ">=0.5",
            "license": "MIT",
            "packages": [
                {
                    "os": "linux",
                    "arch": "amd64",
                    "url": file_url,
                    "sha256": "f7a5a8c16a94fe934007f777a1bf532ef7e42b02133e31abf7523177b220a1ce"
                },
                {
                    "os": "macos",
                    "arch": "aarch64",
                    "url": file_url,
                    "sha256": "f7a5a8c16a94fe934007f777a1bf532ef7e42b02133e31abf7523177b220a1ce"
                },
                {
                    "os": "macos",
                    "arch": "amd64",
                    "url": file_url,
                    "sha256": "f7a5a8c16a94fe934007f777a1bf532ef7e42b02133e31abf7523177b220a1ce"
                }
            ]
        });
        let manifest_file_path = dir.join("example-plugin-manifest.json");
        std::fs::write(
            &manifest_file_path,
            serde_json::to_string(&plugin_manifest_json).unwrap(),
        )?;

        // Install plugin
        let install_args = vec![
            SPIN_BINARY,
            "plugins",
            "install",
            "--file",
            manifest_file_path.to_str().unwrap(),
            "--yes",
        ];
        run(install_args, None, Some(env_map.clone()))?;

        // Execute example plugin which writes "This is an example Spin plugin!" to a specified file
        let execute_args = vec![SPIN_BINARY, "example"];
        let output = run(execute_args, None, Some(env_map.clone()))?;

        // Verify plugin successfully wrote to output file
        assert_eq!(
            std::str::from_utf8(&output.stdout)?.trim(),
            "This is an example Spin plugin!"
        );

        // Upgrade plugin to newer version
        *plugin_manifest_json.get_mut("version").unwrap() = serde_json::json!("0.2.1");
        std::fs::write(
            dir.join("example-plugin-manifest.json"),
            serde_json::to_string(&plugin_manifest_json).unwrap(),
        )?;
        let upgrade_args = vec![
            SPIN_BINARY,
            "plugins",
            "upgrade",
            "example",
            "--file",
            manifest_file_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Cannot convert PathBuf to str"))?,
            "--yes",
        ];
        run(upgrade_args, None, Some(env_map))?;

        // Check plugin version
        let installed_manifest = installed_plugins_dir
            .join("spin")
            .join("plugins")
            .join("manifests")
            .join("example.json");
        let manifest = std::fs::read_to_string(installed_manifest)?;
        assert!(manifest.contains("0.2.1"));

        // Uninstall plugin
        let uninstall_args = vec![SPIN_BINARY, "plugins", "uninstall", "example"];
        run(uninstall_args, None, None)?;
        Ok(())
    }

    // TODO: Test on Windows
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_cloud_plugin_install() -> Result<()> {
        // Create a temporary directory for plugin source and manifests
        let temp_dir = tempdir()?;
        let dir = temp_dir.path();
        let installed_plugins_dir = dir.join("tmp");

        // Ensure that spin installs the plugins into the temporary directory
        let mut env_map: HashMap<&str, &str> = HashMap::new();
        env_map.insert(
            "TEST_PLUGINS_DIRECTORY",
            installed_plugins_dir.to_str().unwrap(),
        );

        // `spin login --help` should cause the `cloud` plugin to be installed
        let args = vec![SPIN_BINARY, "login", "--help"];

        // Execute example plugin which writes "This is an example Spin plugin!" to a specified file
        let output = run(args, None, Some(env_map.clone()))?;

        // Ensure plugin is installed
        assert!(std::str::from_utf8(&output.stdout)?
            .trim()
            .contains("The `cloud` plugin is required. Installing now."));

        // Ensure login help info is displayed
        assert!(std::str::from_utf8(&output.stdout)?
            .trim()
            .contains("Login to Fermyon Cloud"));

        Ok(())
    }

    #[tokio::test]
    async fn test_build_command() -> Result<()> {
        do_test_build_command("tests/build/simple").await
    }

    /// Build an app whose component `workdir` is a subdirectory.
    #[tokio::test]
    #[cfg(not(tarpaulin))]
    async fn test_build_command_nested_workdir() -> Result<()> {
        do_test_build_command("tests/build/nested").await
    }

    /// Build an app whose component `workdir` is a sibling.
    #[tokio::test]
    #[cfg(not(tarpaulin))]
    async fn test_build_command_sibling_workdir() -> Result<()> {
        do_test_build_command("tests/build/sibling").await
    }

    #[tokio::test]
    async fn test_wasi_http_hash_all() -> Result<()> {
        let bodies = Arc::new(
            [
                ("/a", "’Twas brillig, and the slithy toves"),
                ("/b", "Did gyre and gimble in the wabe:"),
                ("/c", "All mimsy were the borogoves,"),
                ("/d", "And the mome raths outgrabe."),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        );

        let server =
            Server::try_bind(&([127, 0, 0, 1], 0).into())?.serve(service::make_service_fn({
                let bodies = bodies.clone();

                move |_| {
                    let bodies = bodies.clone();
                    async move {
                        Ok::<_, Error>(service::service_fn({
                            let bodies = bodies.clone();

                            move |request| {
                                let bodies = bodies.clone();

                                async move {
                                    if let (&Method::GET, Some(body)) =
                                        (request.method(), bodies.get(request.uri().path()))
                                    {
                                        Ok::<_, Error>(Response::new(Body::from(body.to_owned())))
                                    } else {
                                        Response::builder()
                                            .status(StatusCode::METHOD_NOT_ALLOWED)
                                            .body(Body::from(String::new()))
                                    }
                                }
                            }
                        }))
                    }
                }
            }));

        let prefix = format!("http://{}", server.local_addr());

        let (_tx, rx) = oneshot::channel::<()>();

        task::spawn(async move {
            drop(future::select(server, rx).await);
        });

        let controller = SpinTestController::with_manifest(
            "examples/wasi-http-rust-async/spin.toml",
            &[],
            &[],
            None,
        )
        .await?;

        let mut request = Request::get(format!("http://{}/hash-all", controller.url));
        for path in bodies.keys() {
            request = request.header("url", format!("{prefix}{path}"));
        }
        let response = Client::new().request(request.body(Body::empty())?).await?;

        assert_eq!(200, response.status());
        let body = hyper::body::to_bytes(response.into_body()).await?;
        for line in str::from_utf8(&body)?.lines() {
            let (url, hash) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("expected string of form `<url>: <sha-256>`; got {line}"))?;

            let path = url
                .strip_prefix(&prefix)
                .ok_or_else(|| anyhow!("expected string with prefix {prefix}; got {url}"))?;

            let mut hasher = Sha256::new();
            hasher.update(
                bodies
                    .get(path)
                    .ok_or_else(|| anyhow!("unexpected path: {path}"))?,
            );
            assert_eq!(hash, hex::encode(hasher.finalize()));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_wasi_http_echo() -> Result<()> {
        let body = {
            // A sorta-random-ish megabyte
            let mut n = 0_u8;
            iter::repeat_with(move || {
                n = n.wrapping_add(251);
                n
            })
            .take(1024 * 1024)
            .collect::<Vec<_>>()
        };

        let controller = SpinTestController::with_manifest(
            "examples/wasi-http-rust-async/spin.toml",
            &[],
            &[],
            None,
        )
        .await?;

        let response = Client::new()
            .request(
                Request::post(format!("http://{}/echo", controller.url))
                    .header("content-type", "application/octet-stream")
                    .body(Body::wrap_stream(stream::iter(
                        body.chunks(16 * 1024)
                            .map(|chunk| Ok::<_, Error>(Bytes::copy_from_slice(chunk)))
                            .collect::<Vec<_>>(),
                    )))?,
            )
            .await?;

        assert_eq!(200, response.status());
        assert_eq!(
            response.headers()["content-type"],
            "application/octet-stream"
        );
        assert_eq!(body, hyper::body::to_bytes(response.into_body()).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_wasi_messaging() -> Result<()> {
        // In this test, we create a mock Redis server for Spin to talk to.  The mock server only accepts the
        // incoming frames the guest is expected to send, and only sends frames the guest is expected to handle.

        enum Control {
            Continue,
            Stop,
        }

        let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await?;
        let address = listener.local_addr()?;
        let mut futures = FuturesUnordered::new();
        let (mut future_tx, mut future_rx) = mpsc::channel(2);
        let subscribers = Arc::new(AsyncMutex::new(Vec::new()));

        let handle = {
            let subscribers = subscribers.clone();
            move |tx: Arc<AsyncMutex<SplitSink<_, _>>>, frame, address| {
                let subscribers = subscribers.clone();
                async move {
                    let unexpected = || Err(anyhow!("don't know how to handle frame: {frame:?}"));

                    match &frame {
                        Frame::Array { data, .. } => match data.as_slice() {
                            [Frame::BlobString { data, .. }] => match data.deref() {
                                b"PING" => {
                                    tx.lock()
                                        .await
                                        .send(Frame::SimpleString {
                                            data: Bytes::copy_from_slice(b"PONG"),
                                            attributes: None,
                                        })
                                        .await?
                                }
                                _ => return unexpected(),
                            },
                            [Frame::BlobString { data: data1, .. }, Frame::BlobString { data: data2, .. }] => {
                                match (data1.deref(), data2.deref()) {
                                    (b"SUBSCRIBE", b"foo") => {
                                        subscribers.lock().await.push((tx.clone(), address));

                                        let mut tx = tx.lock().await;

                                        tx.send(Frame::Array {
                                            data: vec![
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"subscribe"),
                                                    attributes: None,
                                                },
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"foo"),
                                                    attributes: None,
                                                },
                                                Frame::Number {
                                                    data: 1,
                                                    attributes: None,
                                                },
                                            ],
                                            attributes: None,
                                        })
                                        .await?;

                                        tx.send(Frame::Array {
                                            data: vec![
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"message"),
                                                    attributes: None,
                                                },
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"foo"),
                                                    attributes: None,
                                                },
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"first"),
                                                    attributes: None,
                                                },
                                            ],
                                            attributes: None,
                                        })
                                        .await?;
                                    }
                                    _ => return unexpected(),
                                }
                            }
                            [Frame::BlobString { data: data1, .. }, Frame::BlobString { data: data2, .. }, Frame::BlobString { data: data3, .. }] => {
                                match (data1.deref(), data2.deref(), data3.deref()) {
                                    (b"PUBLISH", b"foo", message) => {
                                        let frame = Frame::Array {
                                            data: vec![
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"message"),
                                                    attributes: None,
                                                },
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(b"foo"),
                                                    attributes: None,
                                                },
                                                Frame::BlobString {
                                                    data: Bytes::copy_from_slice(message),
                                                    attributes: None,
                                                },
                                            ],
                                            attributes: None,
                                        };

                                        let mut count = 0;
                                        for (subscriber, _) in subscribers.lock().await.iter_mut() {
                                            subscriber.lock().await.send(frame.clone()).await?;
                                            count += 1;
                                        }

                                        tx.lock()
                                            .await
                                            .send(Frame::Number {
                                                data: count,
                                                attributes: None,
                                            })
                                            .await?;

                                        if message == b"third" {
                                            return Ok(Control::Stop);
                                        }
                                    }
                                    _ => return unexpected(),
                                }
                            }
                            _ => return unexpected(),
                        },
                        _ => return unexpected(),
                    }

                    Ok(Control::Continue)
                }
            }
        };

        let serve = move |socket, address| {
            let handle = handle.clone();
            async move {
                let (tx, mut rx) = Framed::new(socket, RedisCodec).split();
                let tx = Arc::new(AsyncMutex::new(tx));

                while let Some(frame) = rx.try_next().await? {
                    match handle(tx.clone(), frame, address).await? {
                        Control::Continue => (),
                        Control::Stop => return Ok(Control::Stop),
                    }
                }

                Ok(Control::Continue)
            }
            .boxed()
        };

        futures.push(
            async move {
                loop {
                    let (socket, address) = listener.accept().await?;
                    future_tx.send(serve(socket, address)).await?;
                }
            }
            .boxed(),
        );

        futures.push(
            tokio::process::Command::new(get_process(SPIN_BINARY))
                .arg("up")
                .arg("--follow")
                .arg("wasi-messaging")
                .arg("--file")
                .arg("examples/wasi-messaging-rust/spin.toml")
                .arg("--env")
                .arg(format!("REDIS_ADDRESS=redis://{address}"))
                .env(
                    "RUST_LOG",
                    "spin=trace,spin_loader=trace,spin_core=trace,spin_http=trace",
                )
                .status()
                .map_err(anyhow::Error::from)
                .and_then(|status| {
                    if status.success() {
                        future::ok(Control::Stop)
                    } else {
                        future::err(anyhow!("spin exited with status {status:?}"))
                    }
                })
                .boxed(),
        );

        future::poll_fn(move |cx| loop {
            match Pin::new(&mut futures).poll_next(cx) {
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Err(e)),
                Poll::Ready(None) | Poll::Ready(Some(Ok(Control::Stop))) => {
                    return Poll::Ready(Ok(()))
                }
                _ => (),
            };

            let mut pushed = false;
            while let Ok(Some(future)) = future_rx.try_next() {
                futures.push(future);
                pushed = true;
            }

            if !pushed {
                break Poll::Pending;
            }
        })
        .await
    }
}
