use super::wasi_2023_10_18::{convert, convert_result};
use crate::sockets::{SpinSockets, SpinSocketsView};
use spin_factors::anyhow::Result;
use wasmtime::component::{
    Access, Accessor, FutureReader, Linker, Resource, ResourceTable, StreamReader,
};
use wasmtime_wasi::TrappableError;
use wasmtime_wasi::cli::{WasiCli, WasiCliCtxView};
use wasmtime_wasi::clocks::{WasiClocks, WasiClocksCtxView};
use wasmtime_wasi::filesystem::{WasiFilesystem, WasiFilesystemCtxView};
use wasmtime_wasi::random::{WasiRandom, WasiRandomCtx};
use wasmtime_wasi::sockets::{WasiSockets, WasiSocketsCtxView};

mod latest {
    pub use wasmtime_wasi::p3::bindings::*;
}

mod bindings {
    use super::{TrappableError, latest};

    wasmtime::component::bindgen!({
        path: "../../wit",
        world: "wasi:cli/command@0.3.0-rc-2026-03-15",
        imports: {
            "wasi:cli/stdin": store | trappable,
            "wasi:cli/stdout": store | trappable,
            "wasi:cli/stderr": store | trappable,
            "wasi:filesystem/types.[method]descriptor.read-via-stream": store | trappable,
            "wasi:filesystem/types.[method]descriptor.write-via-stream": store | trappable,
            "wasi:filesystem/types.[method]descriptor.append-via-stream": store | trappable,
            "wasi:filesystem/types.[method]descriptor.read-directory": store | trappable,
            "wasi:sockets/types.[method]tcp-socket.bind": async | trappable,
            "wasi:sockets/types.[method]tcp-socket.listen": async | store | trappable,
            "wasi:sockets/types.[method]tcp-socket.send": store | trappable,
            "wasi:sockets/types.[method]tcp-socket.receive": store | trappable,
            "wasi:sockets/types.[method]udp-socket.bind": async | trappable,
            "wasi:sockets/types.[method]udp-socket.connect": async | trappable,
            default: trappable,
        },
        exports: { default: async | store },
        with: {
            "wasi:cli/terminal-input.terminal-input": latest::cli::terminal_input::TerminalInput,
            "wasi:cli/terminal-output.terminal-output": latest::cli::terminal_output::TerminalOutput,
            "wasi:filesystem/types.descriptor": latest::filesystem::types::Descriptor,
            "wasi:sockets/types.tcp-socket": latest::sockets::types::TcpSocket,
            "wasi:sockets/types.udp-socket": latest::sockets::types::UdpSocket,
        },
    });
}

mod wasi {
    pub use super::bindings::wasi::{
        cli0_3_0_rc_2026_03_15 as cli, clocks0_3_0_rc_2026_03_15 as clocks,
        filesystem0_3_0_rc_2026_03_15 as filesystem, random0_3_0_rc_2026_03_15 as random,
        sockets0_3_0_rc_2026_03_15 as sockets,
    };
}

pub fn add_to_linker<T>(
    linker: &mut Linker<T>,
    io_closure: fn(&mut T) -> &mut ResourceTable,
    random_closure: fn(&mut T) -> &mut WasiRandomCtx,
    clocks_closure: fn(&mut T) -> WasiClocksCtxView<'_>,
    cli_closure: fn(&mut T) -> WasiCliCtxView<'_>,
    filesystem_closure: fn(&mut T) -> WasiFilesystemCtxView<'_>,
    sockets_closure: fn(&mut T) -> SpinSocketsView<'_, T>,
    wasi_sockets_closure: fn(&mut T) -> WasiSocketsCtxView<'_>,
) -> Result<()>
where
    T: Send + 'static,
{
    wasi::clocks::monotonic_clock::add_to_linker::<_, WasiClocks>(linker, clocks_closure)?;
    wasi::clocks::system_clock::add_to_linker::<_, WasiClocks>(linker, clocks_closure)?;
    wasi::filesystem::types::add_to_linker::<_, WasiFilesystem>(linker, filesystem_closure)?;
    wasi::filesystem::preopens::add_to_linker::<_, WasiFilesystem>(linker, filesystem_closure)?;
    wasi::random::random::add_to_linker::<_, WasiRandom>(linker, random_closure)?;
    wasi::random::insecure::add_to_linker::<_, WasiRandom>(linker, random_closure)?;
    wasi::random::insecure_seed::add_to_linker::<_, WasiRandom>(linker, random_closure)?;
    wasi::cli::exit::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::environment::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::stdin::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::stdout::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::stderr::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::terminal_input::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::terminal_output::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::terminal_stdin::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::terminal_stdout::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::cli::terminal_stderr::add_to_linker::<_, WasiCli>(linker, cli_closure)?;
    wasi::sockets::types::add_to_linker::<_, SpinSockets<T>>(linker, sockets_closure)?;
    wasi::sockets::ip_name_lookup::add_to_linker::<_, WasiSockets>(linker, wasi_sockets_closure)?;
    Ok(())
}

impl wasi::clocks::types::Host for WasiClocksCtxView<'_> {}

impl wasi::clocks::system_clock::Host for WasiClocksCtxView<'_> {
    fn now(&mut self) -> wasmtime::Result<wasi::clocks::system_clock::Instant> {
        latest::clocks::system_clock::Host::now(self)
    }

    fn get_resolution(&mut self) -> wasmtime::Result<wasi::clocks::types::Duration> {
        latest::clocks::system_clock::Host::get_resolution(self)
    }
}

impl<T> wasi::clocks::monotonic_clock::HostWithStore<T> for WasiClocks {
    async fn wait_until(
        store: &Accessor<T, Self>,
        when: wasi::clocks::monotonic_clock::Mark,
    ) -> wasmtime::Result<()> {
        latest::clocks::monotonic_clock::HostWithStore::wait_until(store, when).await
    }

    async fn wait_for(
        store: &Accessor<T, Self>,
        duration: wasi::clocks::types::Duration,
    ) -> wasmtime::Result<()> {
        latest::clocks::monotonic_clock::HostWithStore::wait_for(store, duration).await
    }
}

impl wasi::clocks::monotonic_clock::Host for WasiClocksCtxView<'_> {
    fn now(&mut self) -> wasmtime::Result<wasi::clocks::monotonic_clock::Mark> {
        latest::clocks::monotonic_clock::Host::now(self)
    }

    fn get_resolution(&mut self) -> wasmtime::Result<wasi::clocks::types::Duration> {
        latest::clocks::monotonic_clock::Host::get_resolution(self)
    }
}

type FilesystemResult<T> = Result<T, TrappableError<wasi::filesystem::types::ErrorCode>>;

impl wasi::filesystem::types::Host for WasiFilesystemCtxView<'_> {}

impl<T> wasi::filesystem::types::HostDescriptorWithStore<T> for WasiFilesystem {
    fn read_via_stream(
        store: Access<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        offset: wasi::filesystem::types::Filesize,
    ) -> wasmtime::Result<(
        StreamReader<u8>,
        FutureReader<Result<(), wasi::filesystem::types::ErrorCode>>,
    )> {
        latest::filesystem::types::HostDescriptorWithStore::read_via_stream(store, fd, offset)
    }

    fn write_via_stream(
        store: Access<'_, T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        data: StreamReader<u8>,
        offset: wasi::filesystem::types::Filesize,
    ) -> wasmtime::Result<FutureReader<Result<(), wasi::filesystem::types::ErrorCode>>> {
        latest::filesystem::types::HostDescriptorWithStore::write_via_stream(
            store, fd, data, offset,
        )
    }

    fn append_via_stream(
        store: Access<'_, T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        data: StreamReader<u8>,
    ) -> wasmtime::Result<FutureReader<Result<(), wasi::filesystem::types::ErrorCode>>> {
        latest::filesystem::types::HostDescriptorWithStore::append_via_stream(store, fd, data)
    }

    async fn advise(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        offset: wasi::filesystem::types::Filesize,
        length: wasi::filesystem::types::Filesize,
        advice: wasi::filesystem::types::Advice,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::advise(
                store,
                fd,
                offset,
                length,
                advice.into(),
            )
            .await,
        )
    }

    async fn sync_data(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::sync_data(store, fd).await,
        )
    }

    async fn get_flags(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::DescriptorFlags, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::get_flags(store, fd).await,
        )
    }

    async fn get_type(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::DescriptorType, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::get_type(store, fd).await,
        )
    }

    async fn set_size(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        size: wasi::filesystem::types::Filesize,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::set_size(store, fd, size).await,
        )
    }

    async fn set_times(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        data_access_timestamp: wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: wasi::filesystem::types::NewTimestamp,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::set_times(
                store,
                fd,
                data_access_timestamp.into(),
                data_modification_timestamp.into(),
            )
            .await,
        )
    }

    fn read_directory(
        store: Access<'_, T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<(
        StreamReader<wasi::filesystem::types::DirectoryEntry>,
        FutureReader<Result<(), wasi::filesystem::types::ErrorCode>>,
    )> {
        latest::filesystem::types::HostDescriptorWithStore::read_directory(store, fd)
    }

    async fn sync(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(latest::filesystem::types::HostDescriptorWithStore::sync(store, fd).await)
    }

    async fn create_directory_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::create_directory_at(
                store, fd, path,
            )
            .await,
        )
    }

    async fn stat(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::DescriptorStat, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(latest::filesystem::types::HostDescriptorWithStore::stat(store, fd).await)
    }

    async fn stat_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path_flags: wasi::filesystem::types::PathFlags,
        path: String,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::DescriptorStat, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::stat_at(
                store,
                fd,
                path_flags.into(),
                path,
            )
            .await,
        )
    }

    async fn set_times_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path_flags: wasi::filesystem::types::PathFlags,
        path: String,
        data_access_timestamp: wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: wasi::filesystem::types::NewTimestamp,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::set_times_at(
                store,
                fd,
                path_flags.into(),
                path,
                data_access_timestamp.into(),
                data_modification_timestamp.into(),
            )
            .await,
        )
    }

    async fn link_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        old_path_flags: wasi::filesystem::types::PathFlags,
        old_path: String,
        new_fd: Resource<wasi::filesystem::types::Descriptor>,
        new_path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::link_at(
                store,
                fd,
                old_path_flags.into(),
                old_path,
                new_fd,
                new_path.into(),
            )
            .await,
        )
    }

    async fn open_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path_flags: wasi::filesystem::types::PathFlags,
        path: String,
        open_flags: wasi::filesystem::types::OpenFlags,
        flags: wasi::filesystem::types::DescriptorFlags,
    ) -> wasmtime::Result<
        Result<Resource<wasi::filesystem::types::Descriptor>, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::open_at(
                store,
                fd,
                path_flags.into(),
                path,
                open_flags.into(),
                flags.into(),
            )
            .await,
        )
    }

    async fn readlink_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path: String,
    ) -> wasmtime::Result<Result<String, wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::readlink_at(store, fd, path).await,
        )
    }

    async fn remove_directory_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::remove_directory_at(
                store, fd, path,
            )
            .await,
        )
    }

    async fn rename_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        old_path: String,
        new_fd: Resource<wasi::filesystem::types::Descriptor>,
        new_path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::rename_at(
                store, fd, old_path, new_fd, new_path,
            )
            .await,
        )
    }

    async fn symlink_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        old_path: String,
        new_path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::symlink_at(
                store, fd, old_path, new_path,
            )
            .await,
        )
    }

    async fn unlink_file_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path: String,
    ) -> wasmtime::Result<Result<(), wasi::filesystem::types::ErrorCode>> {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::unlink_file_at(store, fd, path)
                .await,
        )
    }

    async fn is_same_object(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        other: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<bool> {
        latest::filesystem::types::HostDescriptorWithStore::is_same_object(store, fd, other).await
    }

    async fn metadata_hash(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::MetadataHashValue, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::metadata_hash(store, fd).await,
        )
    }

    async fn metadata_hash_at(
        store: &Accessor<T, Self>,
        fd: Resource<wasi::filesystem::types::Descriptor>,
        path_flags: wasi::filesystem::types::PathFlags,
        path: String,
    ) -> wasmtime::Result<
        Result<wasi::filesystem::types::MetadataHashValue, wasi::filesystem::types::ErrorCode>,
    > {
        convert_result(
            latest::filesystem::types::HostDescriptorWithStore::metadata_hash_at(
                store,
                fd,
                path_flags.into(),
                path,
            )
            .await,
        )
    }
}

impl wasi::filesystem::types::HostDescriptor for WasiFilesystemCtxView<'_> {
    fn drop(&mut self, fd: Resource<wasi::filesystem::types::Descriptor>) -> wasmtime::Result<()> {
        latest::filesystem::types::HostDescriptor::drop(self, fd)
    }
}

impl wasi::filesystem::preopens::Host for WasiFilesystemCtxView<'_> {
    fn get_directories(
        &mut self,
    ) -> wasmtime::Result<Vec<(Resource<wasi::filesystem::types::Descriptor>, String)>> {
        latest::filesystem::preopens::Host::get_directories(self)
    }
}

convert! {
    enum latest::filesystem::types::ErrorCode => wasi::filesystem::types::ErrorCode {
        Access,
        WouldBlock,
        Already,
        BadDescriptor,
        Busy,
        Deadlock,
        Quota,
        Exist,
        FileTooLarge,
        IllegalByteSequence,
        InProgress,
        Interrupted,
        Invalid,
        Io,
        IsDirectory,
        Loop,
        TooManyLinks,
        MessageSize,
        NameTooLong,
        NoDevice,
        NoEntry,
        NoLock,
        InsufficientMemory,
        InsufficientSpace,
        NotDirectory,
        NotEmpty,
        NotRecoverable,
        Unsupported,
        NoTty,
        NoSuchDevice,
        Overflow,
        NotPermitted,
        Pipe,
        ReadOnly,
        InvalidSeek,
        TextFileBusy,
        CrossDevice,
    }

    enum wasi::filesystem::types::Advice => latest::filesystem::types::Advice {
        Normal,
        Sequential,
        Random,
        WillNeed,
        DontNeed,
        NoReuse,
    }

    flags wasi::filesystem::types::DescriptorFlags [<=>] latest::filesystem::types::DescriptorFlags {
        READ,
        WRITE,
        FILE_INTEGRITY_SYNC,
        DATA_INTEGRITY_SYNC,
        REQUESTED_WRITE_SYNC,
        MUTATE_DIRECTORY,
    }

    enum wasi::filesystem::types::DescriptorType [<=>] latest::filesystem::types::DescriptorType {
        Unknown,
        BlockDevice,
        CharacterDevice,
        Directory,
        Fifo,
        SymbolicLink,
        RegularFile,
        Socket,
    }

    enum wasi::filesystem::types::NewTimestamp => latest::filesystem::types::NewTimestamp {
        NoChange,
        Now,
        Timestamp(e),
    }

    flags wasi::filesystem::types::PathFlags => latest::filesystem::types::PathFlags {
        SYMLINK_FOLLOW,
    }

    flags wasi::filesystem::types::OpenFlags => latest::filesystem::types::OpenFlags {
        CREATE,
        DIRECTORY,
        EXCLUSIVE,
        TRUNCATE,
    }

    struct latest::filesystem::types::MetadataHashValue => wasi::filesystem::types::MetadataHashValue {
        lower,
        upper,
    }

    struct latest::filesystem::types::DirectoryEntry => wasi::filesystem::types::DirectoryEntry {
        type_,
        name,
    }
}

impl From<latest::filesystem::types::DescriptorStat> for wasi::filesystem::types::DescriptorStat {
    fn from(
        e: latest::filesystem::types::DescriptorStat,
    ) -> wasi::filesystem::types::DescriptorStat {
        wasi::filesystem::types::DescriptorStat {
            type_: e.type_.into(),
            link_count: e.link_count,
            size: e.size,
            data_access_timestamp: e.data_access_timestamp.map(|e| e.into()),
            data_modification_timestamp: e.data_modification_timestamp.map(|e| e.into()),
            status_change_timestamp: e.status_change_timestamp.map(|e| e.into()),
        }
    }
}
