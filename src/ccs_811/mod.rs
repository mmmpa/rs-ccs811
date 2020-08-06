mod co2;
mod error;
mod i2c;
mod tvoc;

pub use co2::*;
pub use error::*;
pub use i2c::*;
pub use tvoc::*;

use crate::*;

use nix;
use std::fs::File;
use std::os::unix::io::RawFd;
use std::os::unix::prelude::*;

/// CSS811 specification: https://cdn-learn.adafruit.com/assets/assets/000/044/636/original/CCS811_DS000459_2-00-1098798.pdf

pub const STATUS: I2cRegister = I2cRegister(0x00); // byte: 1  mode: R
pub const MEAS_MODE: I2cRegister = I2cRegister(0x01); // byte: 1 mode: R/W
pub const ALG_RESULT_DATA: I2cRegister = I2cRegister(0x02); // byte: 8 mode: R
pub const RAW_DATA: I2cRegister = I2cRegister(0x03); // byte: 2 mode: R
pub const ENV_DATA: I2cRegister = I2cRegister(0x05); // byte: 4 mode: W
pub const NTC: I2cRegister = I2cRegister(0x06); // byte: 4 mode: R
pub const THRESHOLDS: I2cRegister = I2cRegister(0x10); // byte: 5 mode: W
pub const BASELINE: I2cRegister = I2cRegister(0x11); // byte: 2 mode: R/W
pub const HW_ID: I2cRegister = I2cRegister(0x20); // byte: 1 mode: R
pub const HW_VERSION: I2cRegister = I2cRegister(0x21); // byte: 1 mode: R
pub const FW_BOOT_VERSION: I2cRegister = I2cRegister(0x23); // byte: 2 mode: R
pub const FW_APP_VERSION: I2cRegister = I2cRegister(0x24); // byte: 2 mode: R
pub const ERROR_ID: I2cRegister = I2cRegister(0xE0); // byte: 1 mode: R
pub const APP_START: I2cRegister = I2cRegister(0xF4); // byte: 0 mode: W
pub const SW_RESET: I2cRegister = I2cRegister(0xFF); // byte: 0 mode: W

pub const ERROR: u8 = 0b0000_0001;
pub const DATA_READY: u8 = 0b0000_1000;
pub const APP_VALID: u8 = 0b0001_0000;
pub const FW_MODE: u8 = 0b1000_0000;

pub struct Css811 {
    // File を drop すると close され fd が無効になるので保持しておく
    file: File,
    fd: RawFd,
}

impl Css811 {
    pub fn new(bus: I2cBus, address: I2cAddress) -> Css811Result<Self> {
        let path = format!("/dev/i2c-{}", bus.0);
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        let fd = file.as_raw_fd();

        i2c_slave(fd, address)?;

        Ok(Self { fd, file })
    }

    pub fn start(&self) -> Css811Result<()> {
        i2c_smbus_write_i2c_block_data(self.fd, APP_START, &vec![])?;
        i2c_smbus_write_byte_data(self.fd, MEAS_MODE, 0b11100)?;
        Ok(())
    }

    pub fn status(&self) -> Css811Result<Status> {
        let result = i2c_smbus_read_byte_data(self.fd, STATUS)?;
        Status::new(result)
    }

    pub fn result(&self) -> Css811Result<AlgorithmResultsData> {
        let results = i2c_smbus_read_i2c_block_data(self.fd, ALG_RESULT_DATA, 6)?;
        Ok(AlgorithmResultsData([
            results[0], results[1], results[2], results[3], results[4], results[5],
        ]))
    }

    pub fn error_id(&self) -> Css811Result<ErrorId> {
        let result = i2c_smbus_read_byte_data(self.fd, ERROR_ID)?;
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
