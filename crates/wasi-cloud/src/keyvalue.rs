// use crate::{
//     wasi_http::wasi::http::types2::InputStream,
//     wasi_keyvalue::wasi::io::streams::{self as streams, StreamError, StreamStatus},
//     wasi_keyvalue::{
//         self,
//         wasi::{
//             keyvalue::{
//                 batch::Keys,
//                 readwrite::{Bucket, Error, Host, IncomingValue, Key, OutgoingValue},
//                 types::{IncomingValueAsyncBody, IncomingValueSyncBody, OutputStream},
//             },
//             poll::poll::Pollable,
//         },
//     },
//     WasiCloud,
// };
// use anyhow::{anyhow, Result};
// use spin_core::async_trait;

// #[async_trait]
// impl Host for WasiCloud {
//     async fn get(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//     ) -> Result<Result<IncomingValue, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn set(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//         outgoing_value: OutgoingValue,
//     ) -> Result<Result<(), Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn delete(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//     ) -> Result<Result<(), Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn exists(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//     ) -> Result<Result<bool, Error>, anyhow::Error> {
//         todo!()
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::keyvalue::atomic::Host for WasiCloud {
//     async fn increment(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//         delta: u64,
//     ) -> Result<Result<u64, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn compare_and_swap(
//         &mut self,
//         bucket: Bucket,
//         key: Key,
//         old: u64,
//         new: u64,
//     ) -> Result<Result<bool, Error>, anyhow::Error> {
//         todo!()
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::keyvalue::batch::Host for WasiCloud {
//     async fn get_many(
//         &mut self,
//         bucket: Bucket,
//         keys: Keys,
//     ) -> Result<Result<Vec<IncomingValue>, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn get_keys(&mut self, bucket: Bucket) -> Result<Keys, anyhow::Error> {
//         todo!()
//     }

//     async fn set_many(
//         &mut self,
//         bucket: Bucket,
//         keys: Keys,
//         values: Vec<(Key, OutgoingValue)>,
//     ) -> Result<Result<(), Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn delete_many(
//         &mut self,
//         bucket: Bucket,
//         keys: Keys,
//     ) -> Result<Result<(), Error>, anyhow::Error> {
//         todo!()
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::keyvalue::wasi_cloud_error::Host for WasiCloud {
//     async fn drop_error(&mut self, error: Error) -> Result<(), anyhow::Error> {
//         todo!()
//     }

//     async fn trace(&mut self, error: Error) -> Result<String, anyhow::Error> {
//         todo!()
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::keyvalue::types::Host for WasiCloud {
//     async fn drop_bucket(&mut self, bucket: Bucket) -> Result<(), anyhow::Error> {
//         todo!()
//     }

//     async fn open_bucket(&mut self, name: String) -> Result<Result<Bucket, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn drop_outgoing_value(
//         &mut self,
//         outgoing_value: OutgoingValue,
//     ) -> Result<(), anyhow::Error> {
//         todo!()
//     }

//     async fn new_outgoing_value(&mut self) -> Result<OutgoingValue, anyhow::Error> {
//         todo!()
//     }

//     async fn outgoing_value_write_body(
//         &mut self,
//         outgoing_value: OutgoingValue,
//     ) -> Result<Result<OutputStream, ()>, anyhow::Error> {
//         todo!()
//     }

//     async fn drop_incoming_value(
//         &mut self,
//         incoming_value: IncomingValue,
//     ) -> Result<(), anyhow::Error> {
//         todo!()
//     }

//     async fn incoming_value_consume_sync(
//         &mut self,
//         incoming_value: IncomingValue,
//     ) -> Result<Result<IncomingValueSyncBody, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn incoming_value_consume_async(
//         &mut self,
//         incoming_value: IncomingValue,
//     ) -> Result<Result<IncomingValueAsyncBody, Error>, anyhow::Error> {
//         todo!()
//     }

//     async fn size(&mut self, incoming_value: IncomingValue) -> Result<u64, anyhow::Error> {
//         todo!()
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::io::streams::Host for WasiCloud {
//     async fn read(
//         &mut self,
//         this: InputStream,
//         len: u64,
//     ) -> Result<Result<(Vec<u8>, StreamStatus), StreamError>> {
//         // Ok(self
//         //     .input_streams
//         //     .get_mut(this)
//         //     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//         //     .read(len, Some(self.notify.clone()))
//         //     .await)
//         todo!()
//     }

//     async fn blocking_read(
//         &mut self,
//         this: streams::InputStream,
//         len: u64,
//     ) -> Result<Result<(Vec<u8>, StreamStatus), streams::StreamError>> {
//         // Ok(self
//         //     .input_streams
//         //     .get_mut(this)
//         //     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//         //     .read(len, None)
//         //     .await)
//         todo!()
//     }

//     async fn skip(
//         &mut self,
//         this: streams::InputStream,
//         len: u64,
//     ) -> Result<Result<(u64, StreamStatus), streams::StreamError>> {
//         _ = (this, len);
//         todo!()
//     }

//     async fn blocking_skip(
//         &mut self,
//         this: streams::InputStream,
//         len: u64,
//     ) -> Result<Result<(u64, StreamStatus), streams::StreamError>> {
//         _ = (this, len);
//         todo!()
//     }

//     async fn subscribe_to_input_stream(
//         &mut self,
//         this: streams::InputStream,
//     ) -> Result<streams::Pollable> {
//         self.pollables
//             .push(
//                 self.input_streams
//                     .get_mut(this)
//                     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//                     .pollable
//                     .clone(),
//             )
//             .map_err(|()| anyhow!("table overflow"))
//     }

//     async fn drop_input_stream(&mut self, this: streams::InputStream) -> Result<()> {
//         self.input_streams
//             .remove(this)
//             .map(drop)
//             .ok_or_else(|| anyhow!("unknown handle: {this}"))
//     }

//     async fn write(
//         &mut self,
//         this: streams::OutputStream,
//         buf: Vec<u8>,
//     ) -> Result<Result<u64, streams::StreamError>> {
//         // Ok(self
//         //     .output_streams
//         //     .get_mut(this)
//         //     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//         //     .write(buf, Some(self.notify.clone()))
//         //     .await)
//         todo!()
//     }

//     async fn blocking_write(
//         &mut self,
//         this: streams::OutputStream,
//         buf: Vec<u8>,
//     ) -> Result<Result<u64, streams::StreamError>> {
//         // Ok(self
//         //     .output_streams
//         //     .get_mut(this)
//         //     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//         //     .write(buf, None)
//         //     .await)
//         todo!()
//     }

//     async fn write_zeroes(
//         &mut self,
//         this: streams::OutputStream,
//         len: u64,
//     ) -> Result<Result<u64, streams::StreamError>> {
//         _ = (this, len);
//         todo!()
//     }

//     async fn blocking_write_zeroes(
//         &mut self,
//         this: streams::OutputStream,
//         len: u64,
//     ) -> Result<Result<u64, streams::StreamError>> {
//         _ = (this, len);
//         todo!()
//     }

//     async fn splice(
//         &mut self,
//         this: streams::OutputStream,
//         src: streams::InputStream,
//         len: u64,
//     ) -> Result<Result<(u64, StreamStatus), streams::StreamError>> {
//         _ = (this, src, len);
//         todo!()
//     }

//     async fn blocking_splice(
//         &mut self,
//         this: streams::OutputStream,
//         src: streams::InputStream,
//         len: u64,
//     ) -> Result<Result<(u64, StreamStatus), streams::StreamError>> {
//         _ = (this, src, len);
//         todo!()
//     }

//     async fn forward(
//         &mut self,
//         this: streams::OutputStream,
//         src: streams::InputStream,
//     ) -> Result<Result<u64, streams::StreamError>> {
//         _ = (this, src);
//         todo!()
//     }

//     async fn subscribe_to_output_stream(
//         &mut self,
//         this: streams::OutputStream,
//     ) -> Result<streams::Pollable> {
//         self.pollables
//             .push(
//                 self.output_streams
//                     .get_mut(this)
//                     .ok_or_else(|| anyhow!("unknown handle: {this}"))?
//                     .pollable
//                     .clone(),
//             )
//             .map_err(|()| anyhow!("table overflow"))
//     }

//     async fn drop_output_stream(&mut self, this: streams::OutputStream) -> Result<()> {
//         self.output_streams
//             .remove(this)
//             .map(drop)
//             .ok_or_else(|| anyhow!("unknown handle: {this}"))
//     }
// }

// #[async_trait]
// impl wasi_keyvalue::wasi::poll::poll::Host for WasiCloud {
//     async fn drop_pollable(&mut self, this: Pollable) -> Result<()> {
//         self.pollables.remove(this);
//         Ok(())
//     }

//     async fn poll_oneoff(&mut self, pollables: Vec<Pollable>) -> Result<Vec<bool>> {
//         todo!()
//     }
// }
