//! Socket quota tracking and WASI socket host implementations.
//!
//! This module provides [`SocketPermitState`], [`SpinSocketsView`], and
//! [`SpinSockets`] — the types needed to intercept WASI TCP/UDP socket
//! creation and enforce a per-app cap on the number of concurrently open
//! sockets.

use std::{
    collections::HashMap,
    marker::PhantomData,
    pin::pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use spin_semaphore::{Permit, Semaphore, Type};
use wasmtime::component::{HasData, Resource};
use wasmtime_wasi::p2::bindings::sockets::network as p2_network;
use wasmtime_wasi::p2::bindings::sockets::tcp::{self as p2_tcp, IpSocketAddress, ShutdownType};
use wasmtime_wasi::p2::bindings::sockets::tcp_create_socket as p2_tcp_create;
use wasmtime_wasi::p2::bindings::sockets::udp as p2_udp;
use wasmtime_wasi::p2::bindings::sockets::udp_create_socket as p2_udp_create;
use wasmtime_wasi::p2::{DynInputStream, DynOutputStream, DynPollable};
use wasmtime_wasi::p2::{TcpSocket, UdpSocket};
use wasmtime_wasi::{
    p2::bindings::sockets::network::{ErrorCode as SocketErrorCode, Host as NetworkHost, Network},
    sockets::{WasiSockets, WasiSocketsCtxView},
};

/// A view over WASI socket state that carries an optional per-instance socket
/// permit store, enabling per-connection quota tracking.
pub struct SpinSocketsView<'a, T> {
    pub(crate) inner: WasiSocketsCtxView<'a>,
    pub(crate) semaphore: Semaphore,
    pub(crate) permits: HashMap<u32, Permit>,
    pub(crate) getter: fn(&mut T) -> WasiSocketsCtxView<'_>,
}

impl<'a, T> std::ops::Deref for SpinSocketsView<'a, T> {
    type Target = WasiSocketsCtxView<'a>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for SpinSocketsView<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// [`HasData`] accessor for [`SpinSocketsView`], used in place of [`WasiSockets`]
/// when registering TCP socket bindings so that socket creation and `drop` can
/// participate in socket quota tracking.
pub struct SpinSockets<T>(PhantomData<fn() -> T>);

impl<T: 'static> HasData for SpinSockets<T> {
    type Data<'a> = SpinSocketsView<'a, T>;
}

impl<'a, T> SpinSocketsView<'a, T> {
    /// Consumes this view and returns the inner [`WasiSocketsCtxView`].
    pub fn into_wasi(self) -> WasiSocketsCtxView<'a> {
        self.inner
    }
}

impl<T> p2_tcp::Host for SpinSocketsView<'_, T> {}

impl<T> p2_tcp::HostTcpSocket for SpinSocketsView<'_, T> {
    async fn start_bind(
        &mut self,
        this: Resource<TcpSocket>,
        network: Resource<Network>,
        local_address: IpSocketAddress,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::start_bind(&mut self.inner, this, network, local_address).await
    }

    fn finish_bind(&mut self, this: Resource<TcpSocket>) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::finish_bind(&mut self.inner, this)
    }

    fn start_connect(
        &mut self,
        this: Resource<TcpSocket>,
        network: Resource<Network>,
        remote_address: IpSocketAddress,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::start_connect(&mut self.inner, this, network, remote_address)
    }

    fn finish_connect(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<(Resource<DynInputStream>, Resource<DynOutputStream>)>
    {
        p2_tcp::HostTcpSocket::finish_connect(&mut self.inner, this)
    }

    async fn start_listen(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::start_listen(&mut self.inner, this).await
    }

    fn finish_listen(&mut self, this: Resource<TcpSocket>) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::finish_listen(&mut self.inner, this)
    }

    fn accept(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<(
        Resource<TcpSocket>,
        Resource<DynInputStream>,
        Resource<DynOutputStream>,
    )> {
        // We don't currently support accepting incoming connections.
        _ = this;
        unreachable!()
    }

    fn local_address(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<IpSocketAddress> {
        p2_tcp::HostTcpSocket::local_address(&mut self.inner, this)
    }

    fn remote_address(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<IpSocketAddress> {
        p2_tcp::HostTcpSocket::remote_address(&mut self.inner, this)
    }

    fn is_listening(&mut self, this: Resource<TcpSocket>) -> wasmtime::Result<bool> {
        p2_tcp::HostTcpSocket::is_listening(&mut self.inner, this)
    }

    fn address_family(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime::Result<wasmtime_wasi::p2::bindings::sockets::network::IpAddressFamily> {
        p2_tcp::HostTcpSocket::address_family(&mut self.inner, this)
    }

    fn set_listen_backlog_size(
        &mut self,
        this: Resource<TcpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_listen_backlog_size(&mut self.inner, this, value)
    }

    fn keep_alive_enabled(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<bool> {
        p2_tcp::HostTcpSocket::keep_alive_enabled(&mut self.inner, this)
    }

    fn set_keep_alive_enabled(
        &mut self,
        this: Resource<TcpSocket>,
        value: bool,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_keep_alive_enabled(&mut self.inner, this, value)
    }

    fn keep_alive_idle_time(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_tcp::HostTcpSocket::keep_alive_idle_time(&mut self.inner, this)
    }

    fn set_keep_alive_idle_time(
        &mut self,
        this: Resource<TcpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_keep_alive_idle_time(&mut self.inner, this, value)
    }

    fn keep_alive_interval(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_tcp::HostTcpSocket::keep_alive_interval(&mut self.inner, this)
    }

    fn set_keep_alive_interval(
        &mut self,
        this: Resource<TcpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_keep_alive_interval(&mut self.inner, this, value)
    }

    fn keep_alive_count(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u32> {
        p2_tcp::HostTcpSocket::keep_alive_count(&mut self.inner, this)
    }

    fn set_keep_alive_count(
        &mut self,
        this: Resource<TcpSocket>,
        value: u32,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_keep_alive_count(&mut self.inner, this, value)
    }

    fn hop_limit(&mut self, this: Resource<TcpSocket>) -> wasmtime_wasi::p2::SocketResult<u8> {
        p2_tcp::HostTcpSocket::hop_limit(&mut self.inner, this)
    }

    fn set_hop_limit(
        &mut self,
        this: Resource<TcpSocket>,
        value: u8,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_hop_limit(&mut self.inner, this, value)
    }

    fn receive_buffer_size(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_tcp::HostTcpSocket::receive_buffer_size(&mut self.inner, this)
    }

    fn set_receive_buffer_size(
        &mut self,
        this: Resource<TcpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_receive_buffer_size(&mut self.inner, this, value)
    }

    fn send_buffer_size(
        &mut self,
        this: Resource<TcpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_tcp::HostTcpSocket::send_buffer_size(&mut self.inner, this)
    }

    fn set_send_buffer_size(
        &mut self,
        this: Resource<TcpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::set_send_buffer_size(&mut self.inner, this, value)
    }

    fn subscribe(&mut self, this: Resource<TcpSocket>) -> wasmtime::Result<Resource<DynPollable>> {
        p2_tcp::HostTcpSocket::subscribe(&mut self.inner, this)
    }

    fn shutdown(
        &mut self,
        this: Resource<TcpSocket>,
        shutdown_type: ShutdownType,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_tcp::HostTcpSocket::shutdown(&mut self.inner, this, shutdown_type)
    }

    fn drop(&mut self, this: Resource<TcpSocket>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(&this.rep());
        p2_tcp::HostTcpSocket::drop(&mut self.inner, this)
    }
}

impl<T> NetworkHost for SpinSocketsView<'_, T> {
    fn convert_error_code(
        &mut self,
        error: wasmtime_wasi::p2::SocketError,
    ) -> wasmtime::Result<wasmtime_wasi::p2::bindings::sockets::network::ErrorCode> {
        NetworkHost::convert_error_code(&mut self.inner, error)
    }

    fn network_error_code(
        &mut self,
        err: Resource<wasmtime::Error>,
    ) -> wasmtime::Result<Option<wasmtime_wasi::p2::bindings::sockets::network::ErrorCode>> {
        NetworkHost::network_error_code(&mut self.inner, err)
    }
}

impl<T> wasmtime_wasi::p2::bindings::sockets::network::HostNetwork for SpinSocketsView<'_, T> {
    fn drop(&mut self, this: Resource<Network>) -> wasmtime::Result<()> {
        wasmtime_wasi::p2::bindings::sockets::network::HostNetwork::drop(&mut self.inner, this)
    }
}

impl<T> p2_tcp_create::Host for SpinSocketsView<'_, T> {
    fn create_tcp_socket(
        &mut self,
        address_family: wasmtime_wasi::p2::bindings::sockets::network::IpAddressFamily,
    ) -> wasmtime_wasi::p2::SocketResult<Resource<TcpSocket>> {
        // TODO: Update `wasmtime_wasi`'s bindings generation to generate an
        // async function so we don't have to give up on `Poll::Pending`:
        let permit = match pin!(self.semaphore.acquire(Type::Socket))
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            Poll::Pending => return Err(p2_network::ErrorCode::NewSocketLimit.into()),
            Poll::Ready(result) => result.map_err(|_| p2_network::ErrorCode::NewSocketLimit)?,
        };
        let socket = p2_tcp_create::Host::create_tcp_socket(&mut self.inner, address_family)?;
        self.permits.insert(socket.rep(), permit);
        Ok(socket)
    }
}

impl<T> p2_udp::Host for SpinSocketsView<'_, T> {}

impl<T> p2_udp::HostUdpSocket for SpinSocketsView<'_, T> {
    async fn start_bind(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
        network: Resource<p2_udp::Network>,
        local_address: p2_udp::IpSocketAddress,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_udp::HostUdpSocket::start_bind(&mut self.inner, this, network, local_address).await
    }

    fn finish_bind(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_udp::HostUdpSocket::finish_bind(&mut self.inner, this)
    }

    async fn stream(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
        remote_address: Option<p2_udp::IpSocketAddress>,
    ) -> wasmtime_wasi::p2::SocketResult<(
        Resource<p2_udp::IncomingDatagramStream>,
        Resource<p2_udp::OutgoingDatagramStream>,
    )> {
        p2_udp::HostUdpSocket::stream(&mut self.inner, this, remote_address).await
    }

    fn local_address(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<p2_udp::IpSocketAddress> {
        p2_udp::HostUdpSocket::local_address(&mut self.inner, this)
    }

    fn remote_address(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<p2_udp::IpSocketAddress> {
        p2_udp::HostUdpSocket::remote_address(&mut self.inner, this)
    }

    fn address_family(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime::Result<p2_udp::IpAddressFamily> {
        p2_udp::HostUdpSocket::address_family(&mut self.inner, this)
    }

    fn unicast_hop_limit(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u8> {
        p2_udp::HostUdpSocket::unicast_hop_limit(&mut self.inner, this)
    }

    fn set_unicast_hop_limit(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
        value: u8,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_udp::HostUdpSocket::set_unicast_hop_limit(&mut self.inner, this, value)
    }

    fn receive_buffer_size(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_udp::HostUdpSocket::receive_buffer_size(&mut self.inner, this)
    }

    fn set_receive_buffer_size(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_udp::HostUdpSocket::set_receive_buffer_size(&mut self.inner, this, value)
    }

    fn send_buffer_size(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_udp::HostUdpSocket::send_buffer_size(&mut self.inner, this)
    }

    fn set_send_buffer_size(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
        value: u64,
    ) -> wasmtime_wasi::p2::SocketResult<()> {
        p2_udp::HostUdpSocket::set_send_buffer_size(&mut self.inner, this, value)
    }

    fn subscribe(
        &mut self,
        this: Resource<p2_udp::UdpSocket>,
    ) -> wasmtime::Result<Resource<DynPollable>> {
        p2_udp::HostUdpSocket::subscribe(&mut self.inner, this)
    }

    fn drop(&mut self, this: Resource<p2_udp::UdpSocket>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(&this.rep());
        p2_udp::HostUdpSocket::drop(&mut self.inner, this)
    }
}

impl<T> p2_udp::HostIncomingDatagramStream for SpinSocketsView<'_, T> {
    fn receive(
        &mut self,
        this: Resource<p2_udp::IncomingDatagramStream>,
        max_results: u64,
    ) -> wasmtime_wasi::p2::SocketResult<Vec<p2_udp::IncomingDatagram>> {
        p2_udp::HostIncomingDatagramStream::receive(&mut self.inner, this, max_results)
    }

    fn subscribe(
        &mut self,
        this: Resource<p2_udp::IncomingDatagramStream>,
    ) -> wasmtime::Result<Resource<DynPollable>> {
        p2_udp::HostIncomingDatagramStream::subscribe(&mut self.inner, this)
    }

    fn drop(&mut self, this: Resource<p2_udp::IncomingDatagramStream>) -> wasmtime::Result<()> {
        p2_udp::HostIncomingDatagramStream::drop(&mut self.inner, this)
    }
}

impl<T> p2_udp::HostOutgoingDatagramStream for SpinSocketsView<'_, T> {
    fn check_send(
        &mut self,
        this: Resource<p2_udp::OutgoingDatagramStream>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_udp::HostOutgoingDatagramStream::check_send(&mut self.inner, this)
    }

    fn send(
        &mut self,
        this: Resource<p2_udp::OutgoingDatagramStream>,
        datagrams: Vec<p2_udp::OutgoingDatagram>,
    ) -> wasmtime_wasi::p2::SocketResult<u64> {
        p2_udp::HostOutgoingDatagramStream::send(&mut self.inner, this, datagrams)
    }

    fn subscribe(
        &mut self,
        this: Resource<p2_udp::OutgoingDatagramStream>,
    ) -> wasmtime::Result<Resource<DynPollable>> {
        p2_udp::HostOutgoingDatagramStream::subscribe(&mut self.inner, this)
    }

    async fn drop(
        &mut self,
        this: Resource<p2_udp::OutgoingDatagramStream>,
    ) -> wasmtime::Result<()> {
        p2_udp::HostOutgoingDatagramStream::drop(&mut self.inner, this).await
    }
}

impl<T> p2_udp_create::Host for SpinSocketsView<'_, T> {
    async fn create_udp_socket(
        &mut self,
        address_family: wasmtime_wasi::p2::bindings::sockets::network::IpAddressFamily,
    ) -> wasmtime_wasi::p2::SocketResult<Resource<UdpSocket>> {
        let permit = self
            .semaphore
            .acquire(Type::Socket)
            .await
            .map_err(|_| p2_network::ErrorCode::NewSocketLimit)?;
        let sock = p2_udp_create::Host::create_udp_socket(&mut self.inner, address_family).await?;
        self.permits.insert(sock.rep(), permit);
        Ok(sock)
    }
}

// ===== p3 impls =====

use wasmtime::AsContextMut as _;
use wasmtime::component::{Access, Accessor};
use wasmtime_wasi::p3::bindings::sockets::types::{
    self as p3_types, Duration as p3_Duration, ErrorCode as p3_ErrorCode, Host as p3_Host,
    HostTcpSocket as p3_HostTcpSocket, HostTcpSocketWithStore, HostUdpSocket as p3_HostUdpSocket,
    HostUdpSocketWithStore, IpAddressFamily as p3_IpAddressFamily,
    IpSocketAddress as p3_IpSocketAddress,
};
use wasmtime_wasi::p3::sockets::SocketResult as P3SocketResult;

impl<T> p3_Host for SpinSocketsView<'_, T> {
    fn convert_error_code(
        &mut self,
        error: wasmtime_wasi::p3::sockets::SocketError,
    ) -> wasmtime::Result<p3_ErrorCode> {
        p3_Host::convert_error_code(&mut self.inner, error)
    }
}

impl<T> p3_HostTcpSocket for SpinSocketsView<'_, T> {
    async fn bind(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        local_address: p3_IpSocketAddress,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::bind(&mut self.inner, socket, local_address).await
    }

    fn create(
        &mut self,
        address_family: p3_IpAddressFamily,
    ) -> P3SocketResult<Resource<p3_types::TcpSocket>> {
        // TODO: Update `wasmtime_wasi`'s bindings generation to generate an
        // async function so we don't have to give up on `Poll::Pending`:
        let permit = match pin!(self.semaphore.acquire(Type::Socket))
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            Poll::Pending => {
                return Err(p3_types::ErrorCode::Other(Some(
                    "unable to acquire permit for socket".into(),
                ))
                .into());
            }
            Poll::Ready(result) => {
                result.map_err(|e| p3_types::ErrorCode::Other(Some(e.to_string())))?
            }
        };
        let socket = p3_HostTcpSocket::create(&mut self.inner, address_family)?;
        self.permits.insert(socket.rep(), permit);
        Ok(socket)
    }

    fn get_local_address(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<p3_IpSocketAddress> {
        p3_HostTcpSocket::get_local_address(&mut self.inner, socket)
    }

    fn get_remote_address(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<p3_IpSocketAddress> {
        p3_HostTcpSocket::get_remote_address(&mut self.inner, socket)
    }

    fn get_is_listening(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> wasmtime::Result<bool> {
        p3_HostTcpSocket::get_is_listening(&mut self.inner, socket)
    }

    fn get_address_family(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> wasmtime::Result<p3_IpAddressFamily> {
        p3_HostTcpSocket::get_address_family(&mut self.inner, socket)
    }

    fn set_listen_backlog_size(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: u64,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_listen_backlog_size(&mut self.inner, socket, value)
    }

    fn get_keep_alive_enabled(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<bool> {
        p3_HostTcpSocket::get_keep_alive_enabled(&mut self.inner, socket)
    }

    fn set_keep_alive_enabled(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: bool,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_keep_alive_enabled(&mut self.inner, socket, value)
    }

    fn get_keep_alive_idle_time(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<p3_Duration> {
        p3_HostTcpSocket::get_keep_alive_idle_time(&mut self.inner, socket)
    }

    fn set_keep_alive_idle_time(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: p3_Duration,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_keep_alive_idle_time(&mut self.inner, socket, value)
    }

    fn get_keep_alive_interval(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<p3_Duration> {
        p3_HostTcpSocket::get_keep_alive_interval(&mut self.inner, socket)
    }

    fn set_keep_alive_interval(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: p3_Duration,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_keep_alive_interval(&mut self.inner, socket, value)
    }

    fn get_keep_alive_count(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<u32> {
        p3_HostTcpSocket::get_keep_alive_count(&mut self.inner, socket)
    }

    fn set_keep_alive_count(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: u32,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_keep_alive_count(&mut self.inner, socket, value)
    }

    fn get_hop_limit(&mut self, socket: Resource<p3_types::TcpSocket>) -> P3SocketResult<u8> {
        p3_HostTcpSocket::get_hop_limit(&mut self.inner, socket)
    }

    fn set_hop_limit(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: u8,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_hop_limit(&mut self.inner, socket, value)
    }

    fn get_receive_buffer_size(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<u64> {
        p3_HostTcpSocket::get_receive_buffer_size(&mut self.inner, socket)
    }

    fn set_receive_buffer_size(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: u64,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_receive_buffer_size(&mut self.inner, socket, value)
    }

    fn get_send_buffer_size(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<u64> {
        p3_HostTcpSocket::get_send_buffer_size(&mut self.inner, socket)
    }

    fn set_send_buffer_size(
        &mut self,
        socket: Resource<p3_types::TcpSocket>,
        value: u64,
    ) -> P3SocketResult<()> {
        p3_HostTcpSocket::set_send_buffer_size(&mut self.inner, socket, value)
    }

    fn drop(&mut self, sock: Resource<p3_types::TcpSocket>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(&sock.rep());
        p3_HostTcpSocket::drop(&mut self.inner, sock)
    }
}

impl<T> p3_HostUdpSocket for SpinSocketsView<'_, T> {
    async fn bind(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
        local_address: p3_IpSocketAddress,
    ) -> P3SocketResult<()> {
        p3_HostUdpSocket::bind(&mut self.inner, socket, local_address).await
    }

    async fn connect(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
        remote_address: p3_IpSocketAddress,
    ) -> P3SocketResult<()> {
        p3_HostUdpSocket::connect(&mut self.inner, socket, remote_address).await
    }

    async fn create(
        &mut self,
        address_family: p3_IpAddressFamily,
    ) -> P3SocketResult<Resource<p3_types::UdpSocket>> {
        let permit = self
            .semaphore
            .acquire(Type::Socket)
            .await
            .map_err(|e| p3_types::ErrorCode::Other(Some(e.to_string())))?;
        let sock = p3_HostUdpSocket::create(&mut self.inner, address_family).await?;
        self.permits.insert(sock.rep(), permit);
        Ok(sock)
    }

    fn disconnect(&mut self, socket: Resource<p3_types::UdpSocket>) -> P3SocketResult<()> {
        p3_HostUdpSocket::disconnect(&mut self.inner, socket)
    }

    fn get_local_address(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<p3_IpSocketAddress> {
        p3_HostUdpSocket::get_local_address(&mut self.inner, socket)
    }

    fn get_remote_address(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<p3_IpSocketAddress> {
        p3_HostUdpSocket::get_remote_address(&mut self.inner, socket)
    }

    fn get_address_family(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> wasmtime::Result<p3_IpAddressFamily> {
        p3_HostUdpSocket::get_address_family(&mut self.inner, socket)
    }

    fn get_unicast_hop_limit(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<u8> {
        p3_HostUdpSocket::get_unicast_hop_limit(&mut self.inner, socket)
    }

    fn set_unicast_hop_limit(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
        value: u8,
    ) -> P3SocketResult<()> {
        p3_HostUdpSocket::set_unicast_hop_limit(&mut self.inner, socket, value)
    }

    fn get_receive_buffer_size(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<u64> {
        p3_HostUdpSocket::get_receive_buffer_size(&mut self.inner, socket)
    }

    fn set_receive_buffer_size(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
        value: u64,
    ) -> P3SocketResult<()> {
        p3_HostUdpSocket::set_receive_buffer_size(&mut self.inner, socket, value)
    }

    fn get_send_buffer_size(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<u64> {
        p3_HostUdpSocket::get_send_buffer_size(&mut self.inner, socket)
    }

    fn set_send_buffer_size(
        &mut self,
        socket: Resource<p3_types::UdpSocket>,
        value: u64,
    ) -> P3SocketResult<()> {
        p3_HostUdpSocket::set_send_buffer_size(&mut self.inner, socket, value)
    }

    fn drop(&mut self, sock: Resource<p3_types::UdpSocket>) -> wasmtime::Result<()> {
        let _permit = self.permits.remove(&sock.rep());
        p3_HostUdpSocket::drop(&mut self.inner, sock)
    }
}

impl<T: Send + 'static> HostTcpSocketWithStore<T> for SpinSockets<T> {
    async fn connect(
        store: &Accessor<T, Self>,
        socket: Resource<p3_types::TcpSocket>,
        remote_address: p3_IpSocketAddress,
    ) -> P3SocketResult<()> {
        let getter = store.with(|mut store| store.get().getter);
        let wasi_accessor = store.with_getter::<WasiSockets>(getter);
        <WasiSockets as HostTcpSocketWithStore<T>>::connect(&wasi_accessor, socket, remote_address)
            .await
    }

    async fn listen(
        mut store: Access<'_, T, Self>,
        socket: Resource<p3_types::TcpSocket>,
    ) -> P3SocketResult<wasmtime::component::StreamReader<Resource<p3_types::TcpSocket>>> {
        let getter = store.get().getter;
        let wasi_store = Access::<T, WasiSockets>::new(store.as_context_mut(), getter);
        <WasiSockets as HostTcpSocketWithStore<T>>::listen(wasi_store, socket).await
    }

    fn send(
        mut store: Access<'_, T, Self>,
        socket: Resource<p3_types::TcpSocket>,
        data: wasmtime::component::StreamReader<u8>,
    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), p3_ErrorCode>>> {
        let getter = store.get().getter;
        let wasi_store = Access::<T, WasiSockets>::new(store.as_context_mut(), getter);
        <WasiSockets as HostTcpSocketWithStore<T>>::send(wasi_store, socket, data)
    }

    fn receive(
        mut store: Access<'_, T, Self>,
        socket: Resource<p3_types::TcpSocket>,
    ) -> wasmtime::Result<(
        wasmtime::component::StreamReader<u8>,
        wasmtime::component::FutureReader<Result<(), p3_ErrorCode>>,
    )> {
        let getter = store.get().getter;
        let wasi_store = Access::<T, WasiSockets>::new(store.as_context_mut(), getter);
        <WasiSockets as HostTcpSocketWithStore<T>>::receive(wasi_store, socket)
    }
}

impl<T: 'static> HostUdpSocketWithStore<T> for SpinSockets<T> {
    async fn send(
        store: &Accessor<T, Self>,
        socket: Resource<p3_types::UdpSocket>,
        data: Vec<u8>,
        remote_address: Option<p3_IpSocketAddress>,
    ) -> P3SocketResult<()> {
        let getter = store.with(|mut store| store.get().getter);
        let wasi_accessor = store.with_getter::<WasiSockets>(getter);
        <WasiSockets as HostUdpSocketWithStore<T>>::send(
            &wasi_accessor,
            socket,
            data,
            remote_address,
        )
        .await
    }

    async fn receive(
        store: &Accessor<T, Self>,
        socket: Resource<p3_types::UdpSocket>,
    ) -> P3SocketResult<(Vec<u8>, p3_IpSocketAddress)> {
        let getter = store.with(|mut store| store.get().getter);
        let wasi_accessor = store.with_getter::<WasiSockets>(getter);
        <WasiSockets as HostUdpSocketWithStore<T>>::receive(&wasi_accessor, socket).await
    }
}
