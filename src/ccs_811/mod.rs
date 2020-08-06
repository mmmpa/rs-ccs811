mod co2;
mod error;
mod tvoc;

pub use co2::*;
pub use error::*;
pub use i2c::*;
pub use tvoc::*;

use crate::*;

/// CSS811 specification: https://cdn-learn.adafruit.com/assets/assets/000/044/636/original/CCS811_DS000459_2-00-1098798.pdf

pub const ERROR: u8 = 0b0000_0001;
pub const DATA_READY: u8 = 0b0000_1000;
pub const APP_VALID: u8 = 0b0001_0000;
pub const FW_MODE: u8 = 0b1000_0000;

#[repr(u8)]
pub enum RegisterAddress {
    Status = 0x00,
    MeasMode = 0x01,
    AlgResultData = 0x02,
    RawData = 0x03,
    EnvData = 0x05,
    Ntc = 0x06,
    Thresholds = 0x10,
    Baseline = 0x11,
    HwId = 0x20,
    HwVersion = 0x21,
    FwBootVersion = 0x23,
    FwAppVersion = 0x24,
    ErrorId = 0xE0,
    AppStart = 0xF4,
    SwReset = 0xFF,
}

pub trait I2C {
    fn write_i2c_block_data(&self, reg: RegisterAddress, data: &[u8]) -> Css811Result<()>;
    fn write_byte_data(&self, reg: RegisterAddress, data: u8) -> Css811Result<()>;
    fn read_byte_data(&self, reg: RegisterAddress) -> Css811Result<u8>;
    fn read_i2c_block_data(&self, reg: RegisterAddress, data: &mut [u8]) -> Css811Result<()>;
}

pub trait Ccs811 {
    type I2C: I2C;

    fn i2c(&self) -> &Self::I2C;

    fn start(&self) -> Css811Result<()> {
        self.i2c()
            .write_i2c_block_data(RegisterAddress::AppStart, &vec![])?;
        self.i2c()
            .write_byte_data(RegisterAddress::MeasMode, 0b11100)?;
        Ok(())
    }

    fn status(&self) -> Css811Result<Status> {
        let result = self.i2c().read_byte_data(RegisterAddress::Status)?;
        Status::new(result)
    }

    fn result(&self) -> Css811Result<AlgorithmResultsData> {
        let mut results = [0; 6];
        self.i2c()
            .read_i2c_block_data(RegisterAddress::AlgResultData, &mut results)?;
        Ok(AlgorithmResultsData([
            results[0], results[1], results[2], results[3], results[4], results[5],
        ]))
    }

    fn error_id(&self) -> Css811Result<ErrorId> {
        let result = self.i2c().read_byte_data(RegisterAddress::ErrorId)?;
        let error_id = ErrorId::new(result);
        Ok(error_id)
    }
}

#[derive(Clone, Debug)]
pub struct Status(pub(crate) u8);

impl Status {
    pub fn new(raw: u8) -> Css811Result<Status> {
        let status = Self(raw);

        if status.is_error() {
            Err(Css811Error::ErrorStatus(status))
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
}

// CSS811 による測定値演算結果
pub struct AlgorithmResultsData([u8; 6]);

impl AlgorithmResultsData {
    pub fn status(&self) -> Css811Result<Status> {
        Status::new(self.0[4])
    }
    pub fn co2(&self) -> Co2 {
        Co2(((self.0[0] as u16) << 8 | self.0[1] as u16).into())
    }
    pub fn tvoc(&self) -> Tvoc {
        Tvoc(((self.0[2] as u16) << 8 | self.0[3] as u16).into())
    }
}

mod error_flags {
    pub const MESSAGE: u8 = 0b0000_0001;
    pub const READ_REGISTER: u8 = 0b0000_0010;
    pub const MEASURE_MODE: u8 = 0b0000_0100;
    pub const MAX_RESISTANCE: u8 = 0b0000_1000;
    pub const HEATER_FAULT: u8 = 0b0001_0000;
    pub const HEATER_SUPPLY: u8 = 0b0010_0000;
}

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
            (error_flags::MESSAGE, DeviceError::Message),
            (error_flags::READ_REGISTER, DeviceError::ReadRegister),
            (error_flags::MEASURE_MODE, DeviceError::MeasureMode),
            (error_flags::MAX_RESISTANCE, DeviceError::MaxResistance),
            (error_flags::HEATER_FAULT, DeviceError::HeaterFault),
            (error_flags::HEATER_SUPPLY, DeviceError::HeaterSupply),
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
