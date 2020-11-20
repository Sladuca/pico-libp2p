use crate::crypto::Keypair;
use crate::peer::PeerID;
use crate::stream::Stream;
use std::io::{Read, Write};
use async_trait::async_trait;
use futures::stream::BoxStream;
use parity_multiaddr::Multiaddr;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};

pub mod tcp;
pub mod udp;

/// Struct returned by Transports that allow connection instances to be passed around easily
pub struct BasicConn<Info, Channel: AsyncRead + AsyncWrite> {
    info: Info,
    channel: Channel,
}

/// A basic, non-upgraded connection in libP2P terminology. Roughly corresponds to [this](https://pkg.go.dev/github.com/libp2p/go-libp2p-core/transport#CapableConn)
pub trait BasicConnection {
    fn close();
}

/// Establishes connections, returning [BasicConn](struct.BasicConn.html)s
#[async_trait]
pub trait Transport {
    type Channel: AsyncRead + AsyncWrite;
    type ConnInfo;

    async fn listen<'a>(
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<BasicConn<Self::ConnInfo, Self::Channel>>>>;
    async fn dial(addr: Multiaddr) -> IoResult<BasicConn<Self::ConnInfo, Self::Channel>>;
}