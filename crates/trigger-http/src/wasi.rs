use crate::{HttpExecutor, HttpTrigger};
use anyhow::{anyhow, Context, Result};
use futures::channel::oneshot;
use hyper::{header::HeaderValue, Body, Request, Response};
use spin_core::async_trait;
use spin_trigger::{EitherInstance, TriggerAppEngine};
use std::{
    net::SocketAddr,
    str,
    sync::{Arc, Mutex},
};
use tokio::task;
use wasi_cloud::{
    http::{Fields, IncomingRequest, ResponseOutparam},
    wasi_cloud::{
        wasi::http::types2::{Method, Scheme},
        WasiCloudCore,
    },
};

#[derive(Clone)]
pub struct WasiHttpExecutor;

#[async_trait]
impl HttpExecutor for WasiHttpExecutor {
    async fn execute(
        &self,
        engine: &TriggerAppEngine<HttpTrigger>,
        component_id: &str,
        _base: &str,
        _raw_route: &str,
        req: Request<Body>,
        _client_addr: SocketAddr,
    ) -> Result<Response<Body>> {
        tracing::trace!("Executing request using the WASI executor for component {component_id}",);

        let (instance, mut store) = engine.prepare_instance(component_id).await?;
        let EitherInstance::Component(instance) = instance else {
            unreachable!()
        };

        let proxy = WasiCloudCore::new(&mut store, &instance)?;

        let (response_tx, response_rx) = oneshot::channel();

        let request;
        let response;

        {
            let cloud =
                store
                    .host_components_data()
                    .get_or_insert(engine.wasi_cloud().ok_or_else(|| {
                        anyhow!("WasiHttpExecutor needs access to `wasi-cloud` host component")
                    })?);

            request = cloud.push_incoming_request(IncomingRequest {
                method: match *req.method() {
                    http::Method::GET => Method::Get,
                    http::Method::POST => Method::Post,
                    http::Method::PUT => Method::Put,
                    http::Method::DELETE => Method::Delete,
                    http::Method::PATCH => Method::Patch,
                    http::Method::HEAD => Method::Head,
                    http::Method::OPTIONS => Method::Options,
                    http::Method::TRACE => Method::Trace,
                    ref method => Method::Other(method.as_str().into()),
                },
                path_with_query: req.uri().path_and_query().map(|s| s.as_str().into()),
                scheme: req.uri().scheme().map(|scheme| {
                    if scheme == &http::uri::Scheme::HTTP {
                        Scheme::Http
                    } else if scheme == &http::uri::Scheme::HTTPS {
                        Scheme::Https
                    } else {
                        Scheme::Other(scheme.as_str().into())
                    }
                }),
                authority: req.uri().authority().map(|a| a.as_str().into()),
                headers: Fields(Arc::new(Mutex::new(
                    req.headers()
                        .iter()
                        .map(|(name, value)| (name.to_string(), value.as_bytes().to_vec()))
                        .collect(),
                ))),
                body: Some(req.into_body()),
            })?;

            response = cloud.push_response_outparam(ResponseOutparam(Some(response_tx)))?;
        }

        let handle = task::spawn(async move {
            let result = proxy
                .wasi_http_incoming_handler2()
                .call_handle(&mut store, request, response)
                .await;

            tracing::trace!(
                "memory consumed: {}",
                store.as_ref().data().memory_consumed()
            );

            result
        });

        match response_rx.await {
            Ok(response) => {
                let response = response.context("guest failed to produce a response")?;
                let mut builder = Response::builder().status(response.status);
                for (key, value) in response.headers.0.lock().unwrap().iter() {
                    builder = builder.header(key, HeaderValue::from_bytes(value)?);
                }

                Ok(builder.body(response.body)?)
            }

            Err(_) => {
                handle
                    .await
                    .context("guest invocation panicked")?
                    .context("guest invocation failed")?;

                Err(anyhow!(
                    "guest failed to produce a response prior to returning"
                ))
            }
        }
    }
}
