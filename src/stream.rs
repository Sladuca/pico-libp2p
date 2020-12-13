use bytes::Bytes;
use chrono::{DateTime, Local};
use futures::channel::{mpsc, oneshot};

use crate::errors::{ReadError, WriteError};
use crate::Direction;
pub struct NetStream {
    write_chan: mpsc::Sender<(Bytes, oneshot::Sender<Result<(), WriteError>>)>,
    read_chan: mpsc::Sender<oneshot::Sender<Result<Bytes, ReadError>>>,
}

pub struct StreamInfo {
    id: String,
    direction: Direction,
    opened: DateTime<Local>,
}
