use crate::{DeviceError, Status};

#[derive(Debug)]
pub enum Ccs811Error {
    DeviseError([Option<DeviceError>; 6]),
    #[cfg(feature = "std")]
    I2cError(String),
    ErrorStatus(Status),
}

#[cfg(feature = "std")]
impl std::fmt::Display for Ccs811Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Ccs811Error {}
