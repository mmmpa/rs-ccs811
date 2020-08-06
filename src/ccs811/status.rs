use crate::*;

const ERROR: u8 = 0b0000_0001;
const DATA_READY: u8 = 0b0000_1000;
const APP_VALID: u8 = 0b0001_0000;
const FW_MODE: u8 = 0b1000_0000;

#[derive(Clone, Debug)]
pub struct Status(pub(crate) u8);

impl Status {
    pub fn new(raw: u8) -> Ccs811Result<Status> {
        let status = Self(raw);

        if status.is_error() {
            Err(Ccs811Error::ErrorStatus(status))
        } else {
            Ok(status)
        }
    }

    pub fn is_value(&self) -> bool {
        (self.0 & APP_VALID) != 0
    }

    pub fn is_ready(&self) -> bool {
        (self.0 & DATA_READY) != 0
    }

    pub fn is_error(&self) -> bool {
        (self.0 & ERROR) != 0
    }

    pub fn is_app_mode(&self) -> bool {
        (self.0 & FW_MODE) != 0
    }
}
