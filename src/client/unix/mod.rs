use crate::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct Ccs811Client {
    i2c_cli: LinuxI2CDevice,
}

impl Ccs811Client {
    pub fn new(i2c_cli: LinuxI2CDevice) -> Self {
        Self { i2c_cli }
    }
}

impl I2c for Ccs811Client {
    fn write_i2c_block_data(&mut self, reg: RegisterAddress, data: &[u8]) -> Ccs811Result<()> {
        self.i2c_cli.smbus_write_block_data(reg as u8, data)?;
        Ok(())
    }

    fn write_byte_data(&mut self, reg: RegisterAddress, data: u8) -> Ccs811Result<()> {
        self.i2c_cli.smbus_write_byte_data(reg as u8, data)?;
        Ok(())
    }

    fn read_byte_data(&mut self, reg: RegisterAddress) -> Ccs811Result<u8> {
        let re = self.i2c_cli.smbus_read_byte_data(reg as u8)?;
        Ok(re)
    }

    fn read_i2c_block_data(&mut self, reg: RegisterAddress, data: &mut [u8]) -> Ccs811Result<()> {
        let re = self
            .i2c_cli
            .smbus_read_i2c_block_data(reg as u8, data.len() as u8)?;
        for i in 0..data.len() {
            data[i] = re[i];
        }
        Ok(())
    }
}

impl Ccs811 for Ccs811Client {
    type I2c = Ccs811Client;

    fn i2c(&mut self) -> &mut Self::I2c {
        self
    }
}

impl From<LinuxI2CError> for Ccs811Error {
    fn from(e: LinuxI2CError) -> Self {
        Self::I2cError(e.to_string())
    }
}
