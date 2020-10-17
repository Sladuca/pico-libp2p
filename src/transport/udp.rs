use crate::transport::{Conn, Transport};
use async_trait::async_trait;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::future::FutureExt;
use futures::stream::{once, BoxStream};
use parity_multiaddr::{Multiaddr, Protocol};
use std::net::IpAddr;
use tokio::io::{AsyncRead, AsyncWrite, Error, ErrorKind, Result as IoResult};
use tokio::net::UdpSocket;

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
        } else if let Protocol::Udp(p) = protocol {
            port = Some(p);
            is_valid = true;
            break;
        }
    }
    (is_valid, ip, port)
}

pub struct UdpSocketWrapper(UdpSocket);

impl AsyncRead for UdpSocketWrapper {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<IoResult<usize>> {
        let sock = &Pin::into_inner(self).0;
        sock.poll_recv(cx, buf)
    }
}

impl AsyncWrite for UdpSocketWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let sock = &Pin::into_inner(self).0;
        sock.poll_send(cx, buf).into()
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct UdpTransport {}

pub struct UdpConnInfo {}

#[async_trait]
impl Transport for UdpTransport {
    type ConnInfo = UdpConnInfo;
    type Channel = UdpSocketWrapper;

    async fn listen<'a>(
        addr: Multiaddr,
    ) -> IoResult<BoxStream<'a, IoResult<Conn<Self::ConnInfo, Self::Channel>>>> {
        let (is_valid, ip, port) = is_valid_multiaddress(addr);
        if !is_valid || ip.is_none() || port.is_none() {
            Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - udp multiaddresses must be of the form '/ip4/.../udp/...' or /ip6/.../udp/..."))
        } else {
            let conn_fut = UdpSocket::bind((ip.unwrap(), port.unwrap())).then(|res| async move {
                match res {
                    Ok(sock) => {
                        let wrapped = UdpSocketWrapper(sock);
                        Ok(Conn {
                            channel: wrapped,
                            info: UdpConnInfo {},
                        })
                    }
                    Err(e) => Err(e),
                }
            });
            Ok(Box::pin(once(conn_fut)))
        }
    }

    async fn dial(addr: Multiaddr) -> IoResult<Conn<Self::ConnInfo, Self::Channel>> {
        let (is_valid, ip, port) = is_valid_multiaddress(addr);
        if !is_valid || ip.is_none() || port.is_none() {
            Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - udp multiaddresses must be of the form '/ip4/.../udp/...' or /ip6/.../udp/..."))
        } else {
            let sock = UdpSocket::bind((ip.unwrap(), port.unwrap())).await?;
            let wrapped = UdpSocketWrapper(sock);
            Ok(Conn {
                channel: wrapped,
                info: UdpConnInfo {},
            })
        }
    }
}
