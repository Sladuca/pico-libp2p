use actix::prelude::*;
use futures::stream::BoxStream;
use parity_multiaddr::Multiaddr;

pub struct MsgConnSend(&[u8]);
impl Message for MsgConnSend {
    type Result = Result<bool, std::io::Error>;
}

pub struct MsgConnRecv;
impl Message for MsgConnRecv {
    type Result = Result<&[u8], std::io::Error>;
}

pub struct MsgListen(Multiaddr);
impl Message for MsgListen {
    type Result = Result<BoxStream<Addr<Box<Connection>>>, std::io::Error>;
}

pub struct MsgDial(Multiaddr);
impl Message for MsgDial {
    type Result = Result<Addr<Box<Connection>>, std::io::Error>;
}

/// marker traits that all valid transports must implement
pub trait Connection: Actor + Handler<MsgConnSend> + Handler<MsgConnRecv> {}
pub trait Transport: Actor + Handler<MsgListen> + Handler<MsgDial> {}
