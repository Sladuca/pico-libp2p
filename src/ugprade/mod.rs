use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};
use crate::transport::Conn;
use crate::upgrade::Upgrade;

pub mod switch;

pub struct Switch<U: Upgrade> {
  upgrader: U,
}

#[async_trait]
pub trait Upgrade {
  type Channel: AsyncRead + AsyncWrite;
  type ConnInfo;

  fn upgrade(conn: Conn) -> CapableConn<Info, Channel>;
  fn upgrade_sec(self) -> CapableConn<Info, Channel>;
  fn uggrade_mux(self) -> CapableConn<Info, Channel>;
}
 pub struct CapableConn<Info, Channel: AsyncRead + AsyncWrite> {
  conn: Conn<Info, Channel>,
  local_peer_id: PeerID,
  remote_peer_id: PeerID,
  local_addr: Option<Multiaddr>,
  remote_addr: Option<Multiaddr>,
  keypair: Keypair,
}

#[async_trait]
pub trait Connection: BasicConnection {
    type StreamOtherInfo;
    async fn open_stream() -> IoResult<Stream<Self::StreamOtherInfo>>;
    async fn close_stream(
        stream: Stream<Self::StreamOtherInfo>,
    ) -> IoResult<Stream<Self::StreamOtherInfo>>;
    async fn accept_stream() -> IoResult<Stream<Self::StreamOtherInfo>>;
}