const MESSAGE: u8 = 0b0000_0001;
const READ_REGISTER: u8 = 0b0000_0010;
const MEASURE_MODE: u8 = 0b0000_0100;
const MAX_RESISTANCE: u8 = 0b0000_1000;
const HEATER_FAULT: u8 = 0b0001_0000;
const HEATER_SUPPLY: u8 = 0b0010_0000;

#[derive(Copy, Clone, Debug)]
pub enum DeviceError {
    Message,
    ReadRegister,
    MeasureMode,
    MaxResistance,
    HeaterFault,
    HeaterSupply,
}

pub struct ErrorId(u8, Vec<DeviceError>);

impl ErrorId {
    pub fn new(raw: u8) -> Self {
        let errors = [
            (MESSAGE, DeviceError::Message),
            (READ_REGISTER, DeviceError::ReadRegister),
            (MEASURE_MODE, DeviceError::MeasureMode),
            (MAX_RESISTANCE, DeviceError::MaxResistance),
            (HEATER_FAULT, DeviceError::HeaterFault),
            (HEATER_SUPPLY, DeviceError::HeaterSupply),
        ]
        .iter()
        .filter(|(f, _)| raw & f != 0)
        .map(|(_, e)| *e)
        .collect();

        Self(raw, errors)
    }

    pub fn errors(&self) -> &[DeviceError] {
        &self.1
    }

    pub fn has_error(&self) -> bool {
        !self.1.is_empty()
    }
}
