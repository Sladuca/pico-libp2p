pub mod conn;
pub mod crypto;
pub mod errors;
pub mod stream;
pub mod transport;

pub type StreamID = String;
pub type ConnID = String;
pub type PeerID = String;
pub enum Direction {
    IN,
    OUT,
}
