pub mod error;

use tokio_rustls::{Certificate}
use error::{InvalidKeyError, SigError};
use secrecy::{CloneableSecret, DebugSecret, Secret, Zeroize};
use std::convert::TryFrom;

#[derive(Clone)]
pub enum KeyType {
    RSA = 0,
    Ed25519 = 1,
    Secp256k1 = 2,
    ECDSA = 3,
}

pub trait KeyPair: TryFrom<Vec<u8>, Error = InvalidKeyError> + Into<Vec<u8>> {
    fn verify_sig(&self, data: &[u8], sig: &[u8]) -> Result<bool, SigError>;
    fn sign(&self, data: &[u8]) -> [u8];
    fn get_pub(&self) -> Result<T, InvalidKeyError>;
    fn create_cert(&self) -> Result<Certificate>;
}