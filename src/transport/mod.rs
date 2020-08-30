use async_trait::async_trait;
use futures::stream::BoxStream;
use parity_multiaddr::Multiaddr;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};

pub mod tcp;
pub mod udp;

pub struct Connection<Info, Channel: AsyncRead + AsyncWrite> {
    info: Info,
    channel: Channel,
}

#[async_trait]
pub trait Transport {
    type Channel: AsyncRead + AsyncWrite;
    type ConnInfo;

    async fn listen<'a>(
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<Connection<Self::ConnInfo, Self::Channel>>>>;
    async fn dial(addr: Multiaddr) -> IoResult<Connection<Self::ConnInfo, Self::Channel>>;
}
