
use chrono::{DateTime, Local};
use futures::{
    io::AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt,
    channel::{mpsc, oneshot}
};
use std::sync::Arc;
use crate::errors::{ReadError, WriteError};
use crate::Direction;
use crate::transport::{SecureConnection};
use crate::conn::Multiplex;

pub struct ByteStream {
    rx: Arc<dyn AsyncRead>,
    tx: Arc<dyn AsyncWrite>,
}

pub struct StreamInfo {
    id: String,
    direction: Direction,
    opened: DateTime<Local>,
}

pub struct OpenStream {
    conn: Arc<dyn Multiplex>
}

pub struct AcceptStream {
    conn: Arc<dyn Multiplex>
}

impl Future for OpenStream {
}