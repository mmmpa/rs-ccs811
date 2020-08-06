#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum DeviceError {
    Message = 0b0000_0001,
    ReadRegister = 0b0000_0010,
    MeasureMode = 0b0000_0100,
    MaxResistance = 0b0000_1000,
    HeaterFault = 0b0001_0000,
    HeaterSupply = 0b0010_0000,
}

pub struct ErrorId(u8, [Option<DeviceError>; 6]);

fn to_devise_error(raw: u8, error: DeviceError) -> Option<DeviceError> {
    if raw & error as u8 != 0 {
        Some(error)
    } else {
        None
    }
}

impl ErrorId {
    pub fn new(raw: u8) -> Self {
        let errors = [
            to_devise_error(raw, DeviceError::Message),
            to_devise_error(raw, DeviceError::ReadRegister),
            to_devise_error(raw, DeviceError::MeasureMode),
            to_devise_error(raw, DeviceError::MaxResistance),
            to_devise_error(raw, DeviceError::HeaterFault),
            to_devise_error(raw, DeviceError::HeaterSupply),
        ];

        Self(raw, errors)
    }

    pub fn errors(&self) -> &[Option<DeviceError>] {
        &self.1
    }

    pub fn has_error(&self) -> bool {
        !self.1.is_empty()
    }
}
