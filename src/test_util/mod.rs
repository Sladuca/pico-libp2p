use core::task::{Context, Poll};
use futures::stream::BoxStream;
use std::net::SocketAddr;
use tokio::io::{Error, ErrorKind, Result as IoResult};
use tokio::net::{TcpListener, TcpStream};

pub struct TestTcpListener(TcpListener);

impl TestTcpListener {
    pub fn poll_accept(&mut self, cx: &mut Context) -> Poll<IoResult<(TcpStream, SocketAddr)>> {
        return self.0.poll_accept(cx);
    }
}
