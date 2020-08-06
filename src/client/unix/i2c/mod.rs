mod error;

pub use error::*;

use crate::RegisterAddress;
use std::os::unix::io::RawFd;

/// read: ic2.h and ic2-dev.h

mod read_write {
    pub const I2C_SMBUS_READ: u8 = 1;
    pub const I2C_SMBUS_WRITE: u8 = 0;
}

#[allow(dead_code)]
mod size {
    pub const I2C_SMBUS_QUICK: u32 = 0;
    pub const I2C_SMBUS_BYTE: u32 = 1;
    pub const I2C_SMBUS_BYTE_DATA: u32 = 2;
    pub const I2C_SMBUS_WORD_DATA: u32 = 3;
    pub const I2C_SMBUS_PROC_CALL: u32 = 4;
    pub const I2C_SMBUS_BLOCK_DATA: u32 = 5;
    pub const I2C_SMBUS_I2C_BLOCK_BROKEN: u32 = 6;
    pub const I2C_SMBUS_BLOCK_PROC_CALL: u32 = 7;
    pub const I2C_SMBUS_I2C_BLOCK_DATA: u32 = 8;
}

#[derive(Copy, Clone)]
pub struct I2cBus(pub u8);

#[derive(Copy, Clone)]
pub struct I2cAddress(pub u8);

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct i2c_smbus_ioctl_data {
    pub read_write: u8,
    pub command: u8,
    pub size: u32,
    pub data: *mut i2c_smbus_data,
}

const I2C_SMBUS_BLOCK_MAX: usize = 32;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct i2c_smbus_data {
    pub block: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

impl i2c_smbus_data {
    pub fn empty() -> i2c_smbus_data {
        unsafe { std::mem::zeroed() }
    }
}

// nix の macros は pub で作成してしまうが
// 外で直接 unsafe は使わせないようにする。
#[allow(dead_code)]
mod sealed {
    use crate::client::unix::i2c::i2c_smbus_ioctl_data;

    /// read: ic2.h and ic2-dev.h

    pub const I2C_RETRIES: u32 = 0x0701;
    pub const I2C_TIMEOUT: u32 = 0x0702;
    pub const I2C_SLAVE: u32 = 0x0703;
    pub const I2C_SLAVE_FORCE: u32 = 0x0706;
    pub const I2C_TENBIT: u32 = 0x0704;
    pub const I2C_FUNCS: u32 = 0x0705;
    pub const I2C_RDWR: u32 = 0x0707;
    pub const I2C_PEC: u32 = 0x0708;
    pub const I2C_SMBUS: u32 = 0x0720;

    ioctl_write_int_bad!(i2c_slave_access, I2C_SLAVE);
    ioctl_write_ptr_bad!(i2c_smbus_access, I2C_SMBUS, i2c_smbus_ioctl_data);
}

// https://www.kernel.org/doc/Documentation/i2c/dev-interface

// 実装を追加する時は以下を参考にすること。
// ネーミングもできるだけ一致させる。
//
// https://github.com/mozilla-b2g/i2c-tools/blob/master/lib/smbus.c

type I2CResult<T> = Result<T, I2cError>;

#[allow(dead_code)]
pub fn i2c_slave(fd: RawFd, device_address: I2cAddress) -> I2CResult<()> {
    unsafe { sealed::i2c_slave_access(fd, device_address.0 as i32) }?;
    Ok(())
}

pub fn i2c_smbus_read_byte_data(fd: RawFd, register: RegisterAddress) -> I2CResult<u8> {
    let mut data = i2c_smbus_data::empty();
    let mut message = i2c_smbus_ioctl_data {
        read_write: read_write::I2C_SMBUS_READ,
        command: register as u8,
        size: size::I2C_SMBUS_BYTE_DATA,
        data: &mut data,
    };

    unsafe { sealed::i2c_smbus_access(fd, &mut message) }?;

    Ok(data.block[0])
}

pub fn i2c_smbus_write_byte_data(fd: RawFd, register: RegisterAddress, value: u8) -> I2CResult<u8> {
    let mut data = i2c_smbus_data::empty();
    data.block[0] = value;
    let mut message = i2c_smbus_ioctl_data {
        read_write: read_write::I2C_SMBUS_WRITE,
        command: register as u8,
        size: size::I2C_SMBUS_BYTE_DATA,
        data: &mut data,
    };

    unsafe { sealed::i2c_smbus_access(fd, &mut message) }?;

    Ok(data.block[0])
}

pub fn i2c_smbus_read_i2c_block_data(
    fd: RawFd,
    register: RegisterAddress,
    result: &mut [u8],
) -> I2CResult<()> {
    if result.len() > 255 {
        return Err(I2cError::TooLongBlock);
    }

    let mut data = i2c_smbus_data::empty();
    data.block[0] = result.len() as u8;

    let mut message = i2c_smbus_ioctl_data {
        read_write: read_write::I2C_SMBUS_READ,
        command: register as u8,
        size: size::I2C_SMBUS_I2C_BLOCK_DATA,
        data: &mut data,
    };

    unsafe { sealed::i2c_smbus_access(fd, &mut message) }?;

    let count = data.block[0];
    &data.block[1..(count + 1) as usize]
        .iter()
        .enumerate()
        .for_each(|(i, d)| result[i] = *d);
    Ok(())
}

pub fn i2c_smbus_write_i2c_block_data(
    fd: RawFd,
    register: RegisterAddress,
    values: &[u8],
) -> I2CResult<()> {
    let mut data = i2c_smbus_data::empty();

    data.block[0] = values.len() as u8;
    for (i, value) in values.iter().enumerate() {
        data.block[i + 1] = *value
    }

    let mut message = i2c_smbus_ioctl_data {
        read_write: read_write::I2C_SMBUS_WRITE,
        command: register as u8,
        size: size::I2C_SMBUS_I2C_BLOCK_BROKEN,
        data: &mut data,
    };

    unsafe { sealed::i2c_smbus_access(fd, &mut message) }?;

    Ok(())
}
