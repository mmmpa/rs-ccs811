use crate::Css811Error;
use nix;
use std::process::Output;
use tokio::io::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum I2cError {
    IoctlError(String),
}

impl std::fmt::Display for I2cError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for I2cError {}

impl From<nix::Error> for I2cError {
    fn from(e: nix::Error) -> Self {
        Self::IoctlError(e.to_string())
    }
}

impl Into<Css811Error> for I2cError {
    fn into(self) -> Css811Error {
        Css811Error::I2cError(self.to_string())
    }
}
