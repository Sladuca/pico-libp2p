use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use crate::conn::{Multiplex, ConnInfo};
use crate::crypto::{PubKey, SecretKey};
use crate::errors::{UpgradeError, CloseError};
use crate::{PeerID, StreamID};


pub trait Secure {
    fn local_peer(&self) -> PeerID;
    fn local_sk(&self) -> SecretKey;
    fn remote_peer(&self) -> PeerID;
    fn remote_pk(&self) -> PubKey;
}

pub trait BasicConnection: AsyncRead + AsyncWrite + Unpin {
    fn close(self) -> Result<(), CloseError>;
    fn info(&self) -> ConnInfo;
}

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
