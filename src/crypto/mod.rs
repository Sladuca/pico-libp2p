pub mod error;

use error::{InvalidKeyError, SigError};
use std::convert::TryFrom;

pub enum KeyType {
    RSA = 0,
    Ed25519 = 1,
    Secp256k1 = 2,
    ECDSA = 3,
}

pub struct Key {
    key_type: KeyType,
    bytes: Vec<u8>,
}

pub struct PubKey(Key);
pub struct SecretKey(Key);

pub struct Keypair {
    sk: SecretKey,
    pk: PubKey,
}

pub trait PubKeyTrait: TryFrom<Vec<u8>, Error = InvalidKeyError> + Into<Vec<u8>> {
    fn verify(&self, data: &[u8], sig: &[u8]) -> Result<bool, SigError>;
}

pub trait SecretKeyTrait<T: PubKeyTrait>: TryFrom<Vec<u8>> + Into<Vec<u8>> {
    fn sign(&self, data: &[u8]) -> [u8];
    fn get_pub(&self) -> Result<T, InvalidKeyError>;
}
