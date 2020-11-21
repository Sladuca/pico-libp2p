use crate::crypto::{error::InvalidKeyError, KeyPair, KeyType};
use crate::peer::PeerID;
use crate::stream::{LibP2PStream, Mux};

use async_trait::async_trait;
use futures::stream::BoxStream;
use parity_multiaddr::Multiaddr;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};

pub mod tcp;
pub mod tls;
pub mod udp;

/// A basic, not-necessarily-secure unequiped with a stream multiplexer. That is, a basic connection in libp2p terminology
pub struct BasicConnection<Info, Channel: AsyncRead + AsyncWrite> {
    info: Info,
    channel: Channel,
}

/// A secure conn equipped with a stream multiplexer. That is, a CapableConn in libp2p terminology
pub struct Connection<M: Mux, Info, Channel: AsyncRead + AsyncWrite> {
    conn: BasicConnection<Info, Channel>, // inner BasicConn
    local_peer_id: PeerID,                // this side's PeerID
    remote_peer_id: PeerID,               // other side's PeerID
    local_addr: Option<Multiaddr>,        // this side's multiadress
    remote_addr: Option<Multiaddr>,       // other side's multiaddress
    mux: M,
}

/// Establishes connections, returning [BasicConn](struct.BasicConn.html)s
#[async_trait]
pub trait BasicTransport {
    type Channel: AsyncRead + AsyncWrite;
    type ConnInfo;

    async fn listen<'a>(
        &mut self,
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<BasicConnection<Self::ConnInfo, Self::Channel>>>>;
    async fn dial(
        &mut self,
        addr: Multiaddr,
    ) -> IoResult<BasicConnection<Self::ConnInfo, Self::Channel>>;
}

/// Establishes connections, returning [Connection](struct.Connection.html)s
#[async_trait]
pub trait Transport<M: Mux> {
    type Channel: AsyncRead + AsyncWrite;
    type ConnInfo;

    async fn listen<'a>(
        &mut self,
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<Connection<M, Self::ConnInfo, Self::Channel>>>>;
    async fn dial(
        &mut self,
        addr: Multiaddr,
    ) -> IoResult<Connection<M, Self::ConnInfo, Self::Channel>>;
}

#[async_trait]
pub trait Upgrade<M: Mux>: BasicTransport {
    type T: Transport<
        M,
        ConnInfo = <Self as BasicTransport>::ConnInfo,
        Channel = <Self as BasicTransport>::Channel,
    >;

    fn upgrade(self) -> Self::T;
}
