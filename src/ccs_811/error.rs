use crate::{DeviceError, ErrorId, I2cError, Status};

#[derive(Debug)]
pub enum Css811Error {
    SomethingWrong(String),
    DeviseError(Vec<DeviceError>),
    I2cError(String),
    ErrorStatus(Status),
}

impl std::fmt::Display for Css811Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Css811Error {}

impl From<I2cError> for Css811Error {
    fn from(e: I2cError) -> Self {
        Self::I2cError(e.to_string())
    }
}
