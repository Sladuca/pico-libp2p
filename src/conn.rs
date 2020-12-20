use chrono::{DateTime, Local};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt, Stream,
};
use parity_multiaddr::Multiaddr;

use crate::errors::{
    AcceptStreamError, CloseError, ListenError, OpenStreamError, ReadError, WriteError,
};
use crate::stream::NetStream;
use crate::transport::{BasicChannel, SecureChannel};
use crate::util::ReqRes;
use crate::{ConnID, Direction};

pub struct Conn<C: MuxChannel> {
    conn: C,
    local_addr: Multiaddr,
    remote_addr: Multiaddr,
}

impl<C> Conn<C>
where
    C: MuxChannel + Into<Conn<C>>,
{
    fn close(self) -> Result<(), CloseError> {
        BasicChannel::close(self.conn)
    }
}

impl<C, E> ConnInfoTrait<E> for Conn<C>
where
    C: ConnInfoTrait<E> + MuxChannel,
{
    fn get_conn_info(&self) -> ConnInfo<E> {
        self.conn.get_conn_info()
    }
}

pub trait ConnInfoTrait<E> {
    fn get_conn_info(&self) -> ConnInfo<E>;
}

pub struct ConnInfo<Extra> {
    id: ConnID,
    direction: Direction,
    opened_at: DateTime<Local>,
    extra_info: Extra,
}

pub trait MuxChannel: SecureChannel {
    fn open_stream(&mut self) -> OpenStream;
    fn listen_streams(&mut self) -> Result<ListenStream, ListenError>;
    fn accept_stream(&mut self) -> AcceptStream;
}

pub enum ConnOp {
    AcceptStream,
    OpenStream,
    CloseStream,
    CloseWriteStream,
    CLoseReadStream,
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
        ReqRes::<ConnOp, Result<NetStream, OpenStreamError>>::WaitSend(chan)
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
        ReqRes::<ConnOp, Result<NetStream, AcceptStreamError>>::WaitSend(chan)
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

pub enum ListenStreamState {
    AcceptSend,
    AcceptWait,
}

pub struct ListenStream {
    req_chan: mpsc::Sender<(
        ConnOp,
        oneshot::Sender<Result<NetStream, AcceptStreamError>>,
    )>,
    res_chan: oneshot::Receiver<Result<NetStream, AcceptStreamError>>,
    state: ListenStreamState,
}

impl ListenStream {
    fn new(
        chan: mpsc::Sender<(
            ConnOp,
            oneshot::Sender<Result<NetStream, AcceptStreamError>>,
        )>,
    ) -> Self {
        let (_, rx) = oneshot::channel::<Result<NetStream, AcceptStreamError>>();
        ListenStream {
            req_chan: chan,
            res_chan: rx,
            state: ListenStreamState::AcceptSend,
        }
    }
}

impl Stream for ListenStream {
    type Item = NetStream;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match (&*self).state {
            ListenStreamState::AcceptSend => {
                match self.req_chan.poll_ready(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Err(_)) => Poll::Ready(None),
                    Poll::Ready(Ok(_)) => {
                        let (tx, rx) = oneshot::channel::<Result<NetStream, AcceptStreamError>>();
                        self.res_chan = rx;
                        match self.req_chan.start_send((ConnOp::AcceptStream, tx)) {
                            // send successful, wait for response
                            Ok(_) => {
                                self.state = ListenStreamState::AcceptWait;
                                // poll again with the new state
                                self.poll_next(cx)
                            }

                            // send unsuccessful, return error
                            Err(_) => Poll::Ready(None),
                        }
                    }
                }
            }
            ListenStreamState::AcceptWait => match FutureExt::poll_unpin(&mut (*self).res_chan, cx)
            {
                Poll::Ready(Ok(Ok(res))) => Poll::Ready(Some(res)),
                Poll::Ready(Ok(Err(_))) => Poll::Ready(None),
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
