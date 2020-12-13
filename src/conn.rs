use chrono::{DateTime, Local};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt,
};
use parity_multiaddr::Multiaddr;

use crate::errors::{
    AcceptStreamError, CloseError, ListenError, OpenStreamError, ReadError, WriteError,
};
use crate::stream::NetStream;
use crate::transport::{BasicConnection, FullConnection, SecureConnection};
use crate::{ConnID, Direction};

pub struct Conn<C: SecureConnection> {
    conn: C,
    local_addr: Multiaddr,
    remote_addr: Multiaddr,
}

// impl<C> Conn<C>
// where
//     C: FullConnection,
// {
//     fn close() -> Result<(), CloseError>;
//     fn info<Extra>() -> ConnInfo<Extra>;
//     fn get_stream_infos() -> NetStream;
// }

pub trait Multiplex: SecureConnection {
    fn open_stream() -> OpenStream;
    // fn listen_streams() -> Result<NetStreamStream, ListenError>;
    fn accept_stream() -> AcceptStream;
}

pub struct ConnInfo<Extra> {
    id: ConnID,
    direction: Direction,
    opened: DateTime<Local>,
    extra_info: Extra,
}

pub enum ConnOp {
    AcceptStream,
    OpenStream,
    CloseStream,
    CloseWriteStream,
    CLoseReadStream,
}

pub enum ReqRes<Req, Res> {
    WaitSend(mpsc::Sender<(Req, oneshot::Sender<Res>)>),
    WaitRecv(oneshot::Receiver<Res>),
}

impl<U, T> Unpin for ReqRes<U, T>
where
    U: Unpin,
    T: Unpin,
{
}

pub type OpenStream = ReqRes<ConnOp, Result<NetStream, OpenStreamError>>;

impl OpenStream {
    fn new(
        chan: mpsc::Sender<(ConnOp, oneshot::Sender<Result<NetStream, OpenStreamError>>)>,
    ) -> Self {
        ReqRes::WaitSend(chan)
    }
}

impl Future for OpenStream {
    type Output = Result<NetStream, OpenStreamError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            ReqRes::WaitSend(ref mut chan) => {
                match chan.poll_ready(cx) {
                    // mpsc::Sender is ready, create a oneshot for the new stream and attempt to send
                    Poll::Ready(Ok(_)) => {
                        let (tx, rx) = oneshot::channel::<Self::Output>();
                        match chan.start_send((ConnOp::OpenStream, tx)) {
                            // send successful, wait for response
                            Ok(_) => {
                                self.set(ReqRes::WaitRecv(rx));
                                // poll again with the new state
                                self.poll(cx)
                            }

                            // send unsuccessful, return error
                            Err(_) => Poll::Ready(Err(OpenStreamError {})),
                        }
                    }

                    Poll::Ready(Err(_)) => Poll::Ready(Err(OpenStreamError {})),

                    // mpsc::Sender not ready yet, return pending in WaitSend state
                    Poll::Pending => Poll::Pending,
                }
            }
            ReqRes::WaitRecv(ref mut chan) => {
                match FutureExt::poll_unpin(chan, cx) {
                    // successfully recvd response, return it
                    Poll::Ready(Ok(res)) => Poll::Ready(res),

                    // error when recving response, return error
                    Poll::Ready(Err(_)) => Poll::Ready(Err(OpenStreamError {})),

                    // response not ready yet, return pending in WaitRecv
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

pub type AcceptStream = ReqRes<ConnOp, Result<NetStream, AcceptStreamError>>;

impl AcceptStream {
    fn new(
        chan: mpsc::Sender<(
            ConnOp,
            oneshot::Sender<Result<NetStream, AcceptStreamError>>,
        )>,
    ) -> Self {
        ReqRes::WaitSend(chan)
    }
}

impl Future for AcceptStream {
    type Output = Result<NetStream, AcceptStreamError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            ReqRes::WaitSend(ref mut chan) => {
                match chan.poll_ready(cx) {
                    // mpsc::Sender is ready, create a oneshot for the new stream and attempt to send
                    Poll::Ready(Ok(_)) => {
                        let (tx, rx) = oneshot::channel::<Self::Output>();
                        match chan.start_send((ConnOp::AcceptStream, tx)) {
                            // send successful, wait for response
                            Ok(_) => {
                                self.set(ReqRes::WaitRecv(rx));
                                // poll again with the new state
                                self.poll(cx)
                            }

                            // send unsuccessful, return error
                            Err(_) => Poll::Ready(Err(AcceptStreamError {})),
                        }
                    }

                    Poll::Ready(Err(_)) => Poll::Ready(Err(AcceptStreamError {})),

                    // mpsc::Sender not ready yet, return pending in WaitSend state
                    Poll::Pending => Poll::Pending,
                }
            }
            ReqRes::WaitRecv(ref mut chan) => {
                match FutureExt::poll_unpin(chan, cx) {
                    // successfully recvd response, return it
                    Poll::Ready(Ok(res)) => Poll::Ready(res),

                    // error when recving response, return error
                    Poll::Ready(Err(_)) => Poll::Ready(Err(AcceptStreamError {})),

                    // response not ready yet, return pending in WaitRecv
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}
