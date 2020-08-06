use crate::{DeviceError, Status};

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
