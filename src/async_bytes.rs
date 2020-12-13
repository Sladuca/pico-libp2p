use crate::errors::{ReadError, WriteError};
use bytes::Bytes;
use core::pin::Pin;
use core::task::{Context, Poll};

pub trait AsyncReadBytes {
    // returns the moment bytes are ready with an arbitirary number of bytes
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
}
