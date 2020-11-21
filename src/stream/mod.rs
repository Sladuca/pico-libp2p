use async_trait::async_trait;
use parity_multiaddr::Multiaddr;
use std::boxed::Box;
use std::time;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};

use crate::peer::PeerID;
use crate::transport::Upgrade;
use crate::{Direction, ProtocolID};

pub trait LibP2PStream<StreamInfo>: AsyncWrite + AsyncRead {
    fn local_peer_id(&self) -> PeerID;
    fn remote_peer_id(&self) -> PeerID;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn opened(&self) -> time::Instant;
    fn info(&self) -> StreamInfo;
}

#[async_trait]
pub trait Mux {
    type StreamInfo;

    async fn open_stream(&mut self) -> IoResult<Box<LibP2PStream<Self::StreamInfo>>>;
    async fn close_stream(&mut self, stream: Box<LibP2PStream<Self::StreamInfo>>) -> IoResult<()>;
    async fn accept_stream(&mut self) -> IoResult<Box<LibP2PStream<Self::StreamInfo>>>;
}
