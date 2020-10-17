use crate::transport::{Conn, Transport};
use async_trait::async_trait;
use futures::stream::BoxStream;
use parity_multiaddr::{Multiaddr, Protocol};
use std::net::IpAddr;
use tokio::io::{Error, ErrorKind, Result as IoResult};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;

pub struct TcpConnInfo {}

pub struct TcpTransport {}

fn is_valid_multiaddress(addr: Multiaddr) -> (bool, Option<IpAddr>, Option<u16>) {
    let mut is_valid = false;
    let mut ip: Option<IpAddr> = None;
    let mut port: Option<u16> = None;
    for (i, protocol) in addr.iter().enumerate() {
        if i == 0 {
            match protocol {
                Protocol::Ip4(address) => ip = Some(IpAddr::V4(address)),
                Protocol::Ip6(address) => ip = Some(IpAddr::V6(address)),
                _ => {
                    break;
                }
            };
        } else if let Protocol::Tcp(p) = protocol {
            port = Some(p);
            is_valid = true;
            break;
        }
    }
    (is_valid, ip, port)
}

#[async_trait]
impl Transport for TcpTransport {
    type ConnInfo = TcpConnInfo;
    type Channel = TcpStream;

    async fn listen<'a>(
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<Conn<TcpConnInfo, TcpStream>>>> {
        let (is_valid, ip, port) = is_valid_multiaddress(addr);
        if !is_valid || ip.is_none() || port.is_none() {
            Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - tcp multiaddresses must be of the form '/ip4/.../tcp/...' or /ip6/.../tcp/..."))
        } else {
            // TODO better error handling
            let listener = TcpListener::bind((ip.unwrap(), port.unwrap())).await?;
            let stream = listener.map(|res| {
                match res {
                    Ok(channel) => Ok(Conn {
                        info: TcpConnInfo {}, // TODO: impl a better ConnInfo
                        channel,
                    }),
                    Err(e) => Err(e),
                }
            });
            Ok(Box::pin(stream))
        }
    }
    async fn dial(addr: Multiaddr) -> IoResult<Conn<TcpConnInfo, TcpStream>> {
        let (is_valid, ip, port) = is_valid_multiaddress(addr);
        if !is_valid || ip.is_none() || port.is_none() {
            Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - tcp multiaddresses must be of the form '/ip4/.../tcp/...' or /ip6/.../tcp/..."))
        } else {
            let addr_tup = (ip.unwrap(), port.unwrap());
            let channel = TcpStream::connect(addr_tup).await?;
            Ok(Conn {
                info: TcpConnInfo {},
                channel,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::oneshot;
    #[tokio::test(core_threads = 2)]
    async fn tcp_smoke() {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let listen_addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().unwrap();
            let mut inbound = TcpTransport::listen(listen_addr).await.unwrap();
            let mut count = 0;
            tx.send("ready").unwrap();
            while let Some(conn_result) = inbound.next().await {
                if count >= 10 {
                    break;
                } else {
                    count += 1;
                }
                match conn_result {
                    Ok(mut connection) => {
                        let mut buf: [u8; 6] = [0; 6];
                        let bytes_read = connection.channel.read(&mut buf).await.unwrap();
                        assert_eq!(String::from_utf8_lossy(&buf[0..bytes_read]), "hello!");
                    }
                    Err(e) => panic!(e),
                }
            }
        });

        let handle_2 = tokio::spawn(async move {
            assert_eq!("ready", rx.await.unwrap());
            for _ in 0..10 {
                let dial_addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().unwrap();
                let mut conn = TcpTransport::dial(dial_addr).await.unwrap();
                conn.channel.write_all(b"hello!").await.unwrap();
            }
        });
        handle_2.await.unwrap();
    }
}
