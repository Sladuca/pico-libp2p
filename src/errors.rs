use std::fmt;

#[derive(Debug, Clone)]
pub struct ListenError;

impl fmt::Display for ListenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to listen")
    }
}

#[derive(Debug, Clone)]
pub struct CloseError;

impl fmt::Display for CloseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to close")
    }
}

#[derive(Debug, Clone)]
pub struct WriteError;

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to write")
    }
}

#[derive(Debug, Clone)]
pub struct ReadError;

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to write")
    }
}

#[derive(Debug, Clone)]
pub struct AcceptStreamError;

impl fmt::Display for AcceptStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to accept stream")
    }
}

#[derive(Debug, Clone)]
pub struct OpenStreamError;

impl fmt::Display for OpenStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to open stream")
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeError;

impl fmt::Display for UpgradeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to upgrade")
    }
}
