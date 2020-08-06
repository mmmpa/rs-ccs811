mod i2c;

use crate::*;
use i2c::*;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

pub struct Ccs811Client {
    fd: RawFd,

    // File を drop すると close され fd が無効になるので保持しておく
    #[allow(dead_code)]
    file: File,
}

impl Ccs811Client {
    pub fn new(file: File) -> Self {
        Self {
            fd: file.as_raw_fd(),
            file,
        }
    }
}

impl I2c for Ccs811Client {
    fn write_i2c_block_data(&self, reg: RegisterAddress, data: &[u8]) -> Css811Result<()> {
        i2c_smbus_write_i2c_block_data(self.fd, reg, data)?;
        Ok(())
    }

    fn write_byte_data(&self, reg: RegisterAddress, data: u8) -> Css811Result<()> {
        i2c_smbus_write_byte_data(self.fd, reg, data)?;
        Ok(())
    }

    fn read_byte_data(&self, reg: RegisterAddress) -> Css811Result<u8> {
        let re = i2c_smbus_read_byte_data(self.fd, reg)?;
        Ok(re)
    }

    fn read_i2c_block_data(&self, reg: RegisterAddress, data: &mut [u8]) -> Css811Result<()> {
        i2c_smbus_read_i2c_block_data(self.fd, reg, data)?;
        Ok(())
    }
}

impl Ccs811 for Ccs811Client {
    type I2c = Ccs811Client;

    fn i2c(&self) -> &Self::I2c {
        &self
    }
}
