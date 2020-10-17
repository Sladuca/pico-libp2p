use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};
use parity_multiaddr::Multiaddr;
use crate::crypto::{Keypair};
use crate::transport::{Conn, BasicConnection};
use crate::peer::PeerID;
use crate::stream::Stream;
pub struct Switch<U: Upgrade> {
  upgrader: U,
}

#[async_trait]
pub trait Upgrade {
  type Channel: AsyncRead + AsyncWrite;
  type ConnInfo;

  fn upgrade(conn: Conn<Self::ConnInfo, Self::Channel>) -> CapableConn<Self::ConnInfo, Self::Channel>;
  fn upgrade_sec(self) -> CapableConn<Self::ConnInfo, Self::Channel>;
  fn uggrade_mux(self) -> CapableConn<Self::ConnInfo, Self::Channel>;
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