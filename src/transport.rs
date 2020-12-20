use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::Stream;
use parity_multiaddr::Multiaddr;

use crate::async_bytes::{AsyncReadBytes, AsyncWriteBytes};
use crate::conn::MuxChannel;
use crate::crypto::{PubKey, SecretKey};
use crate::errors::{CloseError, DialError, ListenError, UpgradeError};
use crate::{PeerID, StreamID};

pub trait Transport {
    type Channel: BasicChannel;
    type Dial: Future<Output = Result<Self::Channel, DialError>>;
    type Listen: Stream<Item = Result<Self::Channel, ListenError>>;

    fn is_valid_multiaddr(addr: Multiaddr) -> bool;
    fn dial(addr: Multiaddr) -> Self::Dial;
    fn listen(addrs: Multiaddr) -> Self::Listen;
}

pub trait BasicChannel: AsyncReadBytes + AsyncWriteBytes {
    fn close(self) -> Result<(), CloseError>;
}

pub trait SecureChannel: BasicChannel {
    fn local_peer(&self) -> PeerID;
    fn local_sk(&self) -> SecretKey;
    fn remote_peer(&self) -> PeerID;
    fn remote_pk(&self) -> PubKey;
}

pub trait Upgrade<I> {
    type Output;

    fn poll_upgrade(
        &mut self,
        cx: &mut Context<'_>,
        conn: &mut I,
    ) -> Poll<Result<Self::Output, UpgradeError>>;
}

pub trait SecUpgrade<I: BasicChannel, O: SecureChannel>: Upgrade<I, Output = O> + Default {
    fn sec_upgrade(conn: I) -> UpgradeFut<I, Self> {
        let this: Self = Default::default();
        UpgradeFut::new(this, conn)
    }
}

pub trait MuxUpgrade<I: SecureChannel, O: MuxChannel>: Upgrade<I, Output = O> + Default {
    fn mux_upgrade(conn: I) -> UpgradeFut<I, Self> {
        let this: Self = Default::default();
        UpgradeFut::new(this, conn)
    }
}

pub trait FullUpgrade<I: BasicChannel, O: MuxChannel>: Upgrade<I, Output = O> + Default {
    fn full_upgrade(conn: I) -> UpgradeFut<I, Self> {
        let this: Self = Default::default();
        UpgradeFut::new(this, conn)
    }
}

pub struct UpgradeFut<C, U> {
    upgrader: U,
    upgradee: C,
}

impl<C, U> Unpin for UpgradeFut<C, U> {}

impl<C, U> UpgradeFut<C, U>
where
    U: Upgrade<C>,
{
    pub fn new(upgrader: U, conn: C) -> Self {
        Self {
            upgrader: upgrader,
            upgradee: conn,
        }
    }
}

impl<C, T, U> Future for UpgradeFut<C, U>
where
    U: Upgrade<C, Output = T> + Unpin,
{
    type Output = Result<T, UpgradeError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.upgrader).poll_upgrade(cx, &mut this.upgradee)
    }
}
