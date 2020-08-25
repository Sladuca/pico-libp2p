use crate::transport::{Connection, Transport};
use std::net::IpAddr;
use futures::stream::{BoxStream};
use tokio::io::{Result, Error, ErrorKind};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{StreamExt};
use parity_multiaddr::{Multiaddr, Protocol};
use async_trait::async_trait;

pub struct TcpConnInfo {}

pub struct TcpTransport {}

fn is_valid_multiaddress(addr: Multiaddr) -> (bool, Option<IpAddr>, Option<u16>) {
  let mut is_valid = false;
  let mut ip: Option<IpAddr> = None;
  let mut port: Option<u16> = None;
  for (i, protocol) in addr.iter().enumerate() {
    if i == 0 {
      match protocol {
        Protocol::Ip4(address) => {ip = Some(IpAddr::V4(address))},
        Protocol::Ip6(address) => {ip = Some(IpAddr::V6(address))},
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
impl Transport for TcpListener {
  type ConnInfo = TcpConnInfo;
  type Channel = TcpStream;

  async fn listen<'a>(addr: Multiaddr) -> Result<BoxStream<'a, Result<Connection<Self::ConnInfo, Self::Channel>>>> {
    let (is_valid, ip, port) = is_valid_multiaddress(addr);
    if !is_valid || ip.is_none() || port.is_none() {
      return Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - tcp multiaddresses must be of the form '/ip4/.../tcp/...' or /ip6/.../tcp/..."));
    } else {
      let listener = TcpListener::bind((ip.unwrap(), port.unwrap())).await?;
      let stream = listener.map(|res| {
        match res {
          Ok(channel) => Ok(Connection {
            info: TcpConnInfo {}, // TODO: impl a better ConnInfo
            channel,
          }),
          Err(e) => Err(e),
        }
      });
      Ok(Box::pin(stream))
    }
  }
  async fn dial(addr: Multiaddr) -> Result<Connection<Self::ConnInfo, Self::Channel>> {
    let (is_valid, ip, port) = is_valid_multiaddress(addr);
    if !is_valid || ip.is_none() || port.is_none() {
      return Err(Error::new(ErrorKind::NotFound, "invalid multiaddress - tcp multiaddresses must be of the form '/ip4/.../tcp/...' or /ip6/.../tcp/..."));
    } else {
      let channel = TcpStream::connect((ip.unwrap(), port.unwrap())).await?;
      Ok(Connection {
        info: TcpConnInfo {},
        channel,
      })
    }
  }
}
