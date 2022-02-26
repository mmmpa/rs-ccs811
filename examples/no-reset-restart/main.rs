#![allow(warnings)]

#[macro_use]
extern crate log;

use ccs811::unix::Ccs811Client;
use ccs811::{resume_or_restart, Ccs811, Ccs811Error, Ccs811Result, I2c};
use ccs811::{MeasureDriveMode, MeasureInterrupt, MeasureThresh};
use i2cdev::linux::LinuxI2CDevice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    serve().await
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut dev = Ccs811Client::new_with_path_and_address_hex(
        env!("I2C_DEVICE_PATH"),
        env!("I2C_DEVICE_ADDRESS"),
    )?;

    resume_or_restart(
        &mut dev,
        MeasureDriveMode::EverySecond,
        MeasureInterrupt::Disable,
        MeasureThresh::Disable,
    )
    .await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        match dev.result() {
            Ok(raw) => {
                match raw.status() {
                    Err(e) => {
                        error!("{:?}", e);

                        match dev.error_id() {
                            Ok(id) => {
                                error!("{:?}", id);
                            }
                            Err(e) => {
                                error!("{:?}", e);
                            }
                        }
                        continue;
                    }
                    Ok(s) => {
                        if !s.is_ready() {
                            error!("data is not ready.");

                            match dev.error_id() {
                                Ok(id) => {
                                    error!("{:?}", id);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            }
                            continue;
                        }
                    }
                }

                info!("co2: {:?}, tvoc: {:?}", raw.co2(), raw.tvoc());
            }
            Err(e) => error!("error: {:?}", e),
        }
    }
}
