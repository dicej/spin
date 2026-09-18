use spin_semaphore::{Permit, Semaphore, Type};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use wasmtime::AsContextMut;
use wasmtime::component::{Access, Accessor, FutureReader, HasData, Resource, StreamReader};
use wasmtime_wasi::TrappableError;
use wasmtime_wasi::filesystem::{Descriptor, WasiFilesystem, WasiFilesystemCtxView};
use wasmtime_wasi::p2::bindings::filesystem::types as p2_types;
use wasmtime_wasi::p2::{DynInputStream, DynOutputStream};
use wasmtime_wasi::p3::bindings::filesystem::types as p3_types;

pub struct SpinFilesystemView<'a, T> {
    pub(crate) inner: WasiFilesystemCtxView<'a>,
    pub(crate) semaphore: Semaphore,
    pub(crate) permits: HashMap<u32, Permit>,
    pub(crate) getter: fn(&mut T) -> WasiFilesystemCtxView<'_>,
}

pub struct SpinFilesystem<T>(PhantomData<fn() -> T>);

impl<T: 'static> HasData for SpinFilesystem<T> {
    type Data<'a> = SpinFilesystemView<'a, T>;
}

impl<T> p2_types::Host for SpinFilesystemView<'_, T> {
    fn convert_error_code(
        &mut self,
        error: TrappableError<p2_types::ErrorCode>,
    ) -> wasmtime::Result<p2_types::ErrorCode> {
        p2_types::Host::convert_error_code(&mut self.inner, error)
    }

    fn filesystem_error_code(
        &mut self,
        err: Resource<wasmtime::Error>,
    ) -> wasmtime::Result<Option<p2_types::ErrorCode>> {
        p2_types::Host::filesystem_error_code(&mut self.inner, err)
    }
}

impl<T> p2_types::HostDirectoryEntryStream for SpinFilesystemView<'_, T> {
    async fn read_directory_entry(
        &mut self,
        stream: Resource<p2_types::DirectoryEntryStream>,
    ) -> Result<Option<p2_types::DirectoryEntry>, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDirectoryEntryStream::read_directory_entry(&mut self.inner, stream).await
    }

    fn drop(&mut self, stream: Resource<p2_types::DirectoryEntryStream>) -> wasmtime::Result<()> {
        p2_types::HostDirectoryEntryStream::drop(&mut self.inner, stream)
    }
}

impl<T> p2_types::HostDescriptor for SpinFilesystemView<'_, T> {
    async fn advise(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        offset: p2_types::Filesize,
        len: p2_types::Filesize,
        advice: p2_types::Advice,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::advise(&mut self.inner, fd, offset, len, advice).await
    }

    async fn sync_data(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::sync_data(&mut self.inner, fd).await
    }

    async fn get_flags(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<p2_types::DescriptorFlags, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::get_flags(&mut self.inner, fd).await
    }

    async fn get_type(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<p2_types::DescriptorType, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::get_type(&mut self.inner, fd).await
    }

    async fn set_size(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        size: p2_types::Filesize,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::set_size(&mut self.inner, fd, size).await
    }

    async fn set_times(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        atim: p2_types::NewTimestamp,
        mtim: p2_types::NewTimestamp,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::set_times(&mut self.inner, fd, atim, mtim).await
    }

    async fn read(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        len: p2_types::Filesize,
        offset: p2_types::Filesize,
    ) -> Result<(Vec<u8>, bool), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::read(&mut self.inner, fd, len, offset).await
    }

    async fn write(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        buf: Vec<u8>,
        offset: p2_types::Filesize,
    ) -> Result<p2_types::Filesize, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::write(&mut self.inner, fd, buf, offset).await
    }

    async fn read_directory(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<Resource<p2_types::DirectoryEntryStream>, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::read_directory(&mut self.inner, fd).await
    }

    async fn sync(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::sync(&mut self.inner, fd).await
    }

    async fn create_directory_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::create_directory_at(&mut self.inner, fd, path).await
    }

    async fn stat(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<p2_types::DescriptorStat, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::stat(&mut self.inner, fd).await
    }

    async fn stat_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path_flags: p2_types::PathFlags,
        path: String,
    ) -> Result<p2_types::DescriptorStat, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::stat_at(&mut self.inner, fd, path_flags, path).await
    }

    async fn set_times_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path_flags: p2_types::PathFlags,
        path: String,
        atim: p2_types::NewTimestamp,
        mtim: p2_types::NewTimestamp,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::set_times_at(&mut self.inner, fd, path_flags, path, atim, mtim)
            .await
    }

    async fn link_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        old_path_flags: p2_types::PathFlags,
        old_path: String,
        new_descriptor: Resource<p2_types::Descriptor>,
        new_path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::link_at(
            &mut self.inner,
            fd,
            old_path_flags,
            old_path,
            new_descriptor,
            new_path,
        )
        .await
    }

    async fn open_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path_flags: p2_types::PathFlags,
        path: String,
        oflags: p2_types::OpenFlags,
        flags: p2_types::DescriptorFlags,
    ) -> Result<Resource<p2_types::Descriptor>, TrappableError<p2_types::ErrorCode>> {
        let permit = self
            .semaphore
            .acquire(Type::FileDescriptor)
            .await
            .map_err(|_| {
                // Ideally, we'd return something like POSIX's `ENFILE`, but
                // `wasi:filesystem/types#error-code` does have any equivalent.
                p2_types::ErrorCode::Io
            })?;
        p2_types::HostDescriptor::open_at(&mut self.inner, fd, path_flags, path, oflags, flags)
            .await
            .inspect(|fd| {
                self.permits.insert(fd.rep(), permit);
            })
    }

    fn drop(&mut self, fd: Resource<p2_types::Descriptor>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(fd.rep());
        p2_types::HostDescriptor::drop(&mut self.inner, fd)?;
    }

    async fn readlink_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path: String,
    ) -> Result<String, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::readlink_at(&mut self.inner, fd, path).await
    }

    async fn remove_directory_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::remove_directory_at(&mut self.inner, fd, path).await
    }

    async fn rename_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        old_path: String,
        new_fd: Resource<p2_types::Descriptor>,
        new_path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::rename_at(&mut self.inner, fd, old_path, new_fd, new_path).await
    }

    async fn symlink_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        src_path: String,
        dest_path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::symlink_at(&mut self.inner, fd, src_path, dest_path).await
    }

    async fn unlink_file_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::unlink_file_at(&mut self.inner, fd, path).await
    }

    fn read_via_stream(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        offset: p2_types::Filesize,
    ) -> Result<Resource<DynInputStream>, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::read_via_stream(&mut self.inner, fd, offset)
    }

    fn write_via_stream(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        offset: p2_types::Filesize,
    ) -> Result<Resource<DynOutputStream>, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::write_via_stream(&mut self.inner, fd, offset)
    }

    fn append_via_stream(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<Resource<DynOutputStream>, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::append_via_stream(&mut self.inner, fd)
    }

    async fn is_same_object(
        &mut self,
        a: Resource<p2_types::Descriptor>,
        b: Resource<p2_types::Descriptor>,
    ) -> wasmtime::Result<bool> {
        p2_types::HostDescriptor::is_same_object(&mut self.inner, a, b).await
    }

    async fn metadata_hash(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
    ) -> Result<p2_types::MetadataHashValue, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::metadata_hash(&mut self.inner, fd).await
    }

    async fn metadata_hash_at(
        &mut self,
        fd: Resource<p2_types::Descriptor>,
        path_flags: p2_types::PathFlags,
        path: String,
    ) -> Result<p2_types::MetadataHashValue, TrappableError<p2_types::ErrorCode>> {
        p2_types::HostDescriptor::metadata_hash_at(&mut self.inner, fd, path_flags, path).await
    }
}

impl<T> p3_types::Host for SpinFilesystemView<'_, T> {
    fn convert_error_code(
        &mut self,
        error: TrappableError<p3_types::ErrorCode>,
    ) -> wasmtime::Result<p3_types::ErrorCode> {
        p3_types::Host::convert_error_code(&mut self.inner, error)
    }
}

impl<T: 'static> p3_types::HostDescriptorWithStore<T> for SpinFilesystem<T> {
    fn read_via_stream(
        mut store: Access<T, Self>,
        fd: Resource<Descriptor>,
        offset: p3_types::Filesize,
    ) -> wasmtime::Result<(
        StreamReader<u8>,
        FutureReader<Result<(), p3_types::ErrorCode>>,
    )> {
        let getter = store.get().getter;
        let store = Access::<T, WasiFilesystem>::new(store.as_context_mut(), getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::read_via_stream(store, fd, offset)
    }

    fn write_via_stream(
        mut store: Access<T, Self>,
        fd: Resource<Descriptor>,
        data: StreamReader<u8>,
        offset: p3_types::Filesize,
    ) -> wasmtime::Result<FutureReader<Result<(), p3_types::ErrorCode>>> {
        let getter = store.get().getter;
        let store = Access::<T, WasiFilesystem>::new(store.as_context_mut(), getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::write_via_stream(
            store, fd, data, offset,
        )
    }

    fn append_via_stream(
        mut store: Access<T, Self>,
        fd: Resource<Descriptor>,
        data: StreamReader<u8>,
    ) -> wasmtime::Result<FutureReader<Result<(), p3_types::ErrorCode>>> {
        let getter = store.get().getter;
        let store = Access::<T, WasiFilesystem>::new(store.as_context_mut(), getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::append_via_stream(store, fd, data)
    }

    async fn advise(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        offset: p3_types::Filesize,
        length: p3_types::Filesize,
        advice: p3_types::Advice,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::advise(
            &store, fd, offset, length, advice,
        )
        .await
    }

    async fn sync_data(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::sync_data(&store, fd).await
    }

    async fn get_flags(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<p3_types::DescriptorFlags, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::get_flags(&store, fd).await
    }

    async fn get_type(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<p3_types::DescriptorType, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::get_type(&store, fd).await
    }

    async fn set_size(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        size: p3_types::Filesize,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::set_size(&store, fd, size).await
    }

    async fn set_times(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        data_access_timestamp: p3_types::NewTimestamp,
        data_modification_timestamp: p3_types::NewTimestamp,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::set_times(
            &store,
            fd,
            data_access_timestamp,
            data_modification_timestamp,
        )
        .await
    }

    fn read_directory(
        mut store: Access<T, Self>,
        fd: Resource<Descriptor>,
    ) -> wasmtime::Result<(
        StreamReader<p3_types::DirectoryEntry>,
        FutureReader<Result<(), p3_types::ErrorCode>>,
    )> {
        let getter = store.get().getter;
        let store = Access::<T, WasiFilesystem>::new(store.as_context_mut(), getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::read_directory(store, fd)
    }

    async fn sync(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::sync(&store, fd).await
    }

    async fn create_directory_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::create_directory_at(
            &store, fd, path,
        )
        .await
    }

    async fn stat(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<p3_types::DescriptorStat, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::stat(&store, fd).await
    }

    async fn stat_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path_flags: p3_types::PathFlags,
        path: String,
    ) -> Result<p3_types::DescriptorStat, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::stat_at(
            &store, fd, path_flags, path,
        )
        .await
    }

    async fn set_times_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path_flags: p3_types::PathFlags,
        path: String,
        data_access_timestamp: p3_types::NewTimestamp,
        data_modification_timestamp: p3_types::NewTimestamp,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::set_times_at(
            &store,
            fd,
            path_flags,
            path,
            data_access_timestamp,
            data_modification_timestamp,
        )
        .await
    }

    async fn link_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        old_path_flags: p3_types::PathFlags,
        old_path: String,
        new_fd: Resource<Descriptor>,
        new_path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::link_at(
            &store,
            fd,
            old_path_flags,
            old_path,
            new_fd,
            new_path,
        )
        .await
    }

    async fn open_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path_flags: p3_types::PathFlags,
        path: String,
        open_flags: p3_types::OpenFlags,
        flags: p3_types::DescriptorFlags,
    ) -> Result<Resource<Descriptor>, TrappableError<p3_types::ErrorCode>> {
        let (getter, semaphore) =
            store.with(|mut store| (store.get().getter, store.get().semaphore.clone()));
        let wasi_store = store.with_getter::<WasiFilesystem>(getter);
        let permit = semaphore.acquire(Type::FileDescriptor).await.map_err(|e| {
            // Ideally, we'd return something like POSIX's `ENFILE`, but
            // `wasi:filesystem/types#error-code` does have any equivalent.
            p3_types::ErrorCode::Other(Some(e.to_string()))
        })?;
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::open_at(
            &wasi_store,
            fd,
            path_flags,
            path,
            open_flags,
            flags,
        )
        .await
        .inspect(|fd| {
            store.with(|mut store| store.get().permits.insert(fd.rep(), permit));
        })
    }

    async fn readlink_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path: String,
    ) -> Result<String, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::readlink_at(&store, fd, path)
            .await
    }

    async fn remove_directory_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::remove_directory_at(
            &store, fd, path,
        )
        .await
    }

    async fn rename_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        old_path: String,
        new_fd: Resource<Descriptor>,
        new_path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::rename_at(
            &store, fd, old_path, new_fd, new_path,
        )
        .await
    }

    async fn symlink_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        old_path: String,
        new_path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::symlink_at(
            &store, fd, old_path, new_path,
        )
        .await
    }

    async fn unlink_file_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path: String,
    ) -> Result<(), TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::unlink_file_at(&store, fd, path)
            .await
    }

    async fn is_same_object(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        other: Resource<Descriptor>,
    ) -> wasmtime::Result<bool> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::is_same_object(&store, fd, other)
            .await
    }

    async fn metadata_hash(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
    ) -> Result<p3_types::MetadataHashValue, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::metadata_hash(&store, fd).await
    }

    async fn metadata_hash_at(
        store: &Accessor<T, Self>,
        fd: Resource<Descriptor>,
        path_flags: p3_types::PathFlags,
        path: String,
    ) -> Result<p3_types::MetadataHashValue, TrappableError<p3_types::ErrorCode>> {
        let getter = store.with(|mut store| store.get().getter);
        let store = store.with_getter::<WasiFilesystem>(getter);
        <WasiFilesystem as p3_types::HostDescriptorWithStore<T>>::metadata_hash_at(
            &store, fd, path_flags, path,
        )
        .await
    }
}

impl<T> p3_types::HostDescriptor for SpinFilesystemView<'_, T> {
    fn drop(&mut self, fd: Resource<Descriptor>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(fd.rep());
        p3_types::HostDescriptor::drop(&mut self.inner, fd)
    }
}
