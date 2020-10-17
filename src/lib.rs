pub mod crypto;
pub mod peer;
pub mod stream;
pub mod test_util;
pub mod transport;
pub mod upgrade;

pub type ProtocolID = String;
pub enum Direction {
    IN,
    OUT,
}
