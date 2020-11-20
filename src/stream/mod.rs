use std::time;
use std::boxed::Box;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};
use parity_multiaddr::Multiaddr;

use crate::{Direction, ProtocolID};
use crate::crypto::{KeyPair, KeyType, error::InvalidKeyError};
use crate::transport::{Conn, BasicConnection};
use crate::peer::PeerID;

pub struct Stream<T> {
    id: String,
    protocol: ProtocolID,
    info: StreamInfo<T>,
}

pub struct StreamInfo<T> {
    direction: Direction,
    opened: time::Instant,
    writable: bool
    readable: bool
    other: T,
}

pub struct Conn<Info, Channel: AsyncRead + AsyncWrite> {
    conn: BasicConn<Info, Channel>,
    local_peer_id: PeerID,
    remote_peer_id: PeerID,
    local_addr: Option<Multiaddr>,
    remote_addr: Option<Multiaddr>,
    keypair: Box<dyn KeyPair>,
}

#[async_trait]
pub trait Connection: BasicConnection {
    type StreamInfo;
    type KeyPair;

    async fn open_stream() -> IoResult<Stream<Self::StreamInfo>>;
    async fn close_stream(
        stream: Stream<Self::StreamInfo>,
    ) -> IoResult<Stream<Self::StreamInfo>>;
    async fn accept_stream() -> IoResult<Stream<Self::StreamInfo>>;
}


#[async_trait]
pub trait Upgrade: BasicConnection {
  fn upgrade(self) -> Conn<<Self as BasicConn>::Info>
}