use futures::stream::{BoxStream};
use parity_multiaddr::Multiaddr;
use async_trait::async_trait;
use tokio::io::{AsyncWrite, AsyncRead, Result};

pub mod tcp;

pub struct Connection<Info, Channel: AsyncRead + AsyncWrite> {
    info: Info,
    channel: Channel,
}

#[async_trait]
pub trait Transport {
    type Channel: AsyncRead + AsyncWrite;
    type ConnInfo;

    async fn listen<'a>(addr: Multiaddr) -> Result<BoxStream<'a, Result<Connection<Self::ConnInfo, Self::Channel>>>>;
    async fn dial(addr: Multiaddr) -> Result<Connection<Self::ConnInfo, Self::Channel>>;
}