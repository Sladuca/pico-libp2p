use crate::transport::Transport;

use core::future::Future;
use futures::channel::mpsc;
use futures::future::{ready, Ready};
use parity_multiaddr::Multiaddr;
use std::cell::RefCell;

/// A stub that by default acts like an ideal two-way ordered byte stream and provides
/// methods that allow you to fuck it up in virtually any way you wish.
///
/// stub transport
pub struct StubTransport {}

impl Transport for StubTransport {
    type Channel = StubBasicChannel;
    type Dial = Ready<StubBasicChannel>;
    type Listen = StubListen;

    fn is_valid_multiaddr(_addr: Multiaddr) -> bool {
        true
    }
}

/// stub BasicChannel
pub struct StubBasicChannel {
    tx: mpsc::Sender<u8>,
    rx: mpsc::Receiver<u8>,
}

pub struct StubListen;

/// stub SecureChannel
pub struct StubSecureChannel {
    inner: StubBasicChannel,
}

/// stub MuxChannel
pub struct StubMuxChannel {
    inner: StubSecureChannel,
}

/// stub SecUpgrader
pub struct StubSecUpgrader;

/// stub MuxUgrader
pub struct StubMuxUpgrader;

/// stub FullUgrader
pub struct StubFullUpgrader;
