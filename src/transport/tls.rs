use crate::transport::{BasicConnection, BasicTransport};
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};
use tokio_rustls::rustls::{Certificate, ClientConfig, ServerConfig};

/// a basic connection that has been secured via TLS but still unequipped with a stream multiplexer
pub struct TlsConnection<Info, Channel: AsyncRead + AsyncWrite> {
    cert: Certificate,
    conn: BasicConnection<Info, Channel>,
}

pub struct TlsTransport<T: BasicTransport> {
    clientConfig: ClientConfig,
    ServerConfig: ServerConfig,
    inner_transport: T,
}

// impl<T: BasicTransport> BasicTransport for TlsTransport<T> {
//     type Channel = <T as BasicTransport>::Channel;
//     type ConnInfo = <T as BasicTransport>::ConnInfo;

//     async fn listen<'a>(
//         &mut self,
//         addr: Multiaddr,
//     ) -> IoResult<BoxStream<'a, IoResult<BasicConnection<Self::ConnInfo, Self::Channel>>>> {
//         let inners_stream = self.inner_transport.listen(addr).await?;

//     }
// }
