pub mod async_bytes;
pub mod conn;
pub mod crypto;
pub mod errors;
pub mod stream;
pub mod transport;
pub mod util;

pub type StreamID = String;
pub type ConnID = String;
pub type PeerID = String;
pub type ProtocolID = String;

pub enum Direction {
    IN,
    OUT,
}
