use crate::transport::{Connection, Transport};
use async_trait::async_trait;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::stream::BoxStream;
use parity_multiaddr::{Multiaddr, Protocol};
use std::net::IpAddr;
use tokio::io::{AsyncRead, AsyncWrite, Error, ErrorKind, Result as IoResult};
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;

pub struct UdpTransport {}

pub struct UdpConnInfo {}

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
