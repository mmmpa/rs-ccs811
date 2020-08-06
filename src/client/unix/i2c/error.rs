use crate::Ccs811Error;

#[derive(Debug, Eq, PartialEq)]
pub enum I2cError {
    IoctlError(String),
    TooLongBlock,
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

impl From<I2cError> for Ccs811Error {
    fn from(e: I2cError) -> Self {
        Self::I2cError(e.to_string())
    }
}
