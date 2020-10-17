use crate::{Direction, ProtocolID};
use std::time;

pub struct Stream<T> {
    id: String,
    protocol: ProtocolID,
    info: StreamInfo<T>,
}

pub struct StreamInfo<T> {
    direction: Direction,
    opened: time::Instant,
    other: T,
}
