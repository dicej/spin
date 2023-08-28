use spin_core::async_trait;

use crate::{
    wasi_http::{
        self,
        exports::wasi,
        wasi::http::{
            readwrite::{Bucket, Error, IncomingValue, Key, OutgoingValue},
            types_keyvalue::{IncomingValueAsyncBody, IncomingValueSyncBody, OutputStream},
        },
    },
    WasiCloud,
};

#[async_trait]
impl crate::wasi_http::wasi::http::readwrite::Host for WasiCloud {
    async fn get(
        &mut self,
        bucket: Bucket,
        key: Key,
    ) -> Result<Result<IncomingValue, Error>, anyhow::Error> {
        todo!()
    }

    async fn set(
        &mut self,
        bucket: Bucket,
        key: Key,
        outgoing_value: OutgoingValue,
    ) -> Result<Result<(), Error>, anyhow::Error> {
        todo!()
    }

    async fn delete(
        &mut self,
        bucket: Bucket,
        key: Key,
    ) -> Result<Result<(), Error>, anyhow::Error> {
        todo!()
    }

    async fn exists(
        &mut self,
        bucket: Bucket,
        key: Key,
    ) -> Result<Result<bool, Error>, anyhow::Error> {
        todo!()
    }
}

#[async_trait]
impl wasi_http::wasi::http::wasi_cloud_error::Host for WasiCloud {
    async fn drop_error(&mut self, error: Error) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn trace(&mut self, error: Error) -> Result<String, anyhow::Error> {
        todo!()
    }
}

#[async_trait]
impl wasi_http::wasi::http::types_keyvalue::Host for WasiCloud {
    async fn drop_bucket(&mut self, bucket: Bucket) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn open_bucket(&mut self, name: String) -> Result<Result<Bucket, Error>, anyhow::Error> {
        todo!()
    }

    async fn drop_outgoing_value(
        &mut self,
        outgoing_value: OutgoingValue,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn new_outgoing_value(&mut self) -> Result<OutgoingValue, anyhow::Error> {
        todo!()
    }

    async fn outgoing_value_write_body_async(
        &mut self,
        outgoing_value: OutgoingValue,
    ) -> Result<Result<OutputStream, Error>, anyhow::Error> {
        todo!()
    }

    async fn outgoing_value_write_body_sync(
        &mut self,
        outgoing_value: OutgoingValue,
        value: Vec<u8>,
    ) -> Result<Result<(), Error>, anyhow::Error> {
        todo!()
    }

    async fn drop_incoming_value(
        &mut self,
        incoming_value: IncomingValue,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn incoming_value_consume_sync(
        &mut self,
        incoming_value: IncomingValue,
    ) -> Result<Result<IncomingValueSyncBody, Error>, anyhow::Error> {
        todo!()
    }

    async fn incoming_value_consume_async(
        &mut self,
        incoming_value: IncomingValue,
    ) -> Result<Result<IncomingValueAsyncBody, Error>, anyhow::Error> {
        todo!()
    }

    async fn size(&mut self, incoming_value: IncomingValue) -> Result<u64, anyhow::Error> {
        todo!()
    }
}
