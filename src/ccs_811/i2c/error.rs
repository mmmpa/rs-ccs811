use nix;
use std::process::Output;
use tokio::io::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum I2cError {
    IoctlError(String),
}

#[derive(Debug)]
pub struct RunCommandError {
    pub command: String,
    pub output: Output,
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
