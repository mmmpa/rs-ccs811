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

/// CSS811 specification: https://cdn-learn.adafruit.com/assets/assets/000/044/636/original/CCS811_DS000459_2-00-1098798.pdf

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
        Ok(AlgorithmResultsData::new([
            results[0], results[1], results[2], results[3], results[4], results[5],
        ]))
    }

    fn error_id(&self) -> Css811Result<ErrorId> {
        let result = self.i2c().read_byte_data(RegisterAddress::ErrorId)?;
        let error_id = ErrorId::new(result);
        Ok(error_id)
    }
}
