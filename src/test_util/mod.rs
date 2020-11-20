use crate::transport::{BasicConnection, Transport};
use core::task::{Context, Poll};
use futures::stream::BoxStream;
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite, Error, ErrorKind, Result as IoResult};

struct TestTransport<T: Transport>(T);
struct TestChannel<C: AsyncRead + AsyncWrite>(C);
