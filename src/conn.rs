use chrono::{DateTime, Local};
use core::future::Future;
use core::pin::Pin;
use std::sync::Arc;
use core::task::{Context, Poll};
use futures::{
    channel::{mpsc, oneshot},
    io::{AsyncRead, AsyncWrite, Error},
    FutureExt,
};
use parity_multiaddr::Multiaddr;

use crate::errors::{
    AcceptStreamError, CloseError, ListenError, OpenStreamError, ReadError, WriteError,
};
use crate::stream::{ByteStream, OpenStream, AcceptStream};
use crate::transport::{BasicConnection, FullConnection, SecureConnection};
use crate::{ConnID, Direction};

pub struct Conn<C: SecureConnection> {
    conn: C,
}

impl<C: SecureConnection> AsyncRead for Conn<C> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<Result<usize, Error>> {
        // let this = &mut *self;
        // Pin::new(&mut this.conn).poll_read(cx, buf)
        let conn = unsafe { self.map_unchecked_mut(|s| &mut s.conn ) };
        conn.poll_read(cx, buf)
    }
}



impl<C> Conn<C>
where
    C: FullConnection,
{
    fn info(&self) -> ConnInfo {
        self.conn.info()
    }
}

pub trait Multiplex: AsyncRead + AsyncWrite {
    fn open_stream(self: Arc<Self>) -> OpenStream;
    // fn listen_streams() -> Result<NetStreamStream, ListenError>;
    fn accept_stream(self: Arc<Self>) -> AcceptStream;
}

pub struct ConnInfo {
    id: ConnID,
    direction: Direction,
    opened: DateTime<Local>,
    local_addr: Multiaddr,
    remote_addr: Multiaddr,
}

pub enum ConnOp {
    AcceptStream,
    OpenStream,
    CloseStream,
    CloseWriteStream,
    CLoseReadStream,
}

