mod algorithm_results_data;
mod co2;
mod error;
mod error_id;
mod status;
mod tvoc;

pub use algorithm_results_data::*;
pub use co2::*;
pub use error::*;
pub use error_id::*;
pub use status::*;
pub use tvoc::*;

use crate::*;

/// CCS811 specification: https://cdn-learn.adafruit.com/assets/assets/000/044/636/original/CCS811_DS000459_2-00-1098798.pdf

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

#[repr(u8)]
pub enum MeasureDriveMode {
    Idle = 0b0000_0000,
    EverySecond = 0b0001_0000,
    EveryTenSeconds = 0b0010_0000,
    EveryMinute = 0b0011_0000,
    Raw = 0b0100_1000, // no algorithm results data
}

#[repr(u8)]
pub enum MeasureInterrupt {
    Enable = 0b0000_1000,
    Disable = 0b0000_0000,
}

#[repr(u8)]
pub enum MeasureThresh {
    Enable = 0b0000_0100,
    Disable = 0b0000_0000,
}

pub trait I2c {
    fn write_i2c_blank_data(&mut self, reg: RegisterAddress) -> Ccs811Result<()>;
    fn write_byte_data(&mut self, reg: RegisterAddress, data: u8) -> Ccs811Result<()>;
    fn read_byte_data(&mut self, reg: RegisterAddress) -> Ccs811Result<u8>;
    fn read_i2c_block_data(&mut self, reg: RegisterAddress, data: &mut [u8]) -> Ccs811Result<()>;
}

pub trait Ccs811 {
    type I2c: I2c;

    fn i2c(&mut self) -> &mut Self::I2c;

    fn start(
        &mut self,
        mode: MeasureDriveMode,
        interrupt: MeasureInterrupt,
        thresh: MeasureThresh,
    ) -> Ccs811Result<()> {
        self.i2c().write_i2c_blank_data(RegisterAddress::AppStart)?;
        self.i2c().write_byte_data(
            RegisterAddress::MeasMode,
            mode as u8 | interrupt as u8 | thresh as u8,
        )?;
        Ok(())
    }

    fn status(&mut self) -> Ccs811Result<Status> {
        let result = self.i2c().read_byte_data(RegisterAddress::Status)?;
        Status::new(result)
    }

    fn result(&mut self) -> Ccs811Result<AlgorithmResultsData> {
        let mut results = [0; 6];
        self.i2c()
            .read_i2c_block_data(RegisterAddress::AlgResultData, &mut results)?;
        Ok(AlgorithmResultsData::new([
            results[0], results[1], results[2], results[3], results[4], results[5],
        ]))
    }

    fn error_id(&mut self) -> Ccs811Result<ErrorId> {
        let result = self.i2c().read_byte_data(RegisterAddress::ErrorId)?;
        let error_id = ErrorId::new(result);
        Ok(error_id)
    }
}
