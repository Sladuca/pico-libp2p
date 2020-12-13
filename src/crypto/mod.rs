pub mod errors;

use errors::{InvalidKeyError, SigError};
use std::convert::TryFrom;

pub type PubKey = Vec<u8>;
pub type SecretKey = Vec<u8>;

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
    fn get_pub(&self) -> Result<PubKey, InvalidKeyError>;
    fn get_priv(&self) -> Result<SecretKey, InvalidKeyError>;
}
