use crate::errors::{ReadError, WriteError};
use bytes::Bytes;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::io::{AsyncBufRead, AsyncRead, AsyncWrite, Error, ErrorKind};

pub trait AsyncReadBytes {
    // returns once bytes are available, returning a reference to those bytes in a Bytes struct
    fn poll_read_bytes(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Bytes, ReadError>>;

    // returns once it read n bytes or the stream closed
    fn poll_readn_bytes(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        n: usize,
    ) -> Poll<Result<Bytes, ReadError>>;
}

pub trait AsyncWriteBytes {
    fn poll_write_bytes(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: Bytes,
    ) -> Poll<Result<(), WriteError>>;

    fn poll_flush_bytes(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>;

    fn poll_close_bytes(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>>;
}
