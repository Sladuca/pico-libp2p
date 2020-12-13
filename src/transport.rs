use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::async_bytes::{AsyncReadBytes, AsyncWriteBytes};
use crate::conn::Multiplex;
use crate::crypto::{PubKey, SecretKey};
use crate::errors::UpgradeError;
use crate::{PeerID, StreamID};

pub trait Secure {
    fn local_peer() -> PeerID;
    fn local_sk() -> SecretKey;
    fn remote_peer() -> PeerID;
    fn remote_pk() -> PubKey;
}

pub trait BasicConnection: AsyncReadBytes + AsyncWriteBytes {}

pub trait SecureConnection: BasicConnection + Secure {
    fn upgrade<'a, C: 'a + BasicConnection>(conn: C) -> UpgradeFut<'a, C, Self>;
}

pub trait FullConnection: SecureConnection + Multiplex {
    fn upgrade_full<'a, C: 'a + BasicConnection>(conn: C) -> UpgradeFut<'a, C, Self>;
    fn upgrade_secure<'a, C: 'a + SecureConnection>(conn: C) -> UpgradeFut<'a, C, Self>;
}

pub trait Upgrade<Input> {
    type Output;

    fn poll_upgrade(
        self: &mut Self,
        cx: &mut Context<'_>,
        conn: &mut Input,
    ) -> Poll<Result<Self::Output, UpgradeError>>;
}

pub struct UpgradeFut<'a, C, U: ?Sized> {
    upgrader: &'a mut U,
    upgradee: &'a mut C,
}

impl<'a, C, U> UpgradeFut<'a, C, U> {
    pub fn new(upgrader: &'a mut U, conn: &'a mut C) -> Self {
        Self {
            upgrader: upgrader,
            upgradee: conn,
        }
    }
}

impl<C, T, U> Future for UpgradeFut<'_, C, U>
where
    U: Upgrade<C, Output = T> + ?Sized + Unpin,
{
    type Output = Result<T, UpgradeError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.upgrader).poll_upgrade(cx, this.upgradee)
    }
}
