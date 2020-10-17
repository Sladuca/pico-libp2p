use std::fmt;

#[derive(Debug, Clone)]
pub enum SigError {}

impl fmt::Display for SigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unspecified signature error"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InvalidKeyError {}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unspecified invalid key error"),
        }
    }
}
