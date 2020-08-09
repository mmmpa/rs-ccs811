# ccs811

This is to use CCS811 with Rust.

[SparkFun Air Quality Breakout \- CCS811 \- SEN\-14193 \- SparkFun Electronics](https://www.sparkfun.com/products/14193)

```sh
export I2C_DEVICE_NUMBER="1"
export CCS811_I2C_DEVICE_ADDRESS="0x5b"
```

```toml
ccs811 = { git = "https://github.com/mmmpa/rs_ccs811", features = ["std"] }
```

```rust
use ccs811::unix::Ccs811Client;
use ccs811::{Ccs811, MeasureDriveMode, MeasureInterrupt, MeasureThresh};

fn ccs811_device_address() -> u16 {
    let no_prefix = env!("CCS811_I2C_DEVICE_ADDRESS").trim_start_matches("0x");
    u16::from_str_radix(no_prefix, 16).unwrap()
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let i2c_cli = LinuxI2CDevice::new(
        format!("/dev/i2c-{}", env!("I2C_DEVICE_NUMBER")),
        ccs811_device_address(),
    )
    .unwrap();
    let mut dev = Ccs811Client::new(i2c_cli);

    dev.start(
        MeasureDriveMode::EverySecond,
        MeasureInterrupt::Enable,
        MeasureThresh::Enable,
    )?;

    loop {
        tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;

        match dev.result() {
            Ok(raw) => {
                match raw.status() {
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    }
                    Ok(s) => {
                        if !s.is_ready() {
                            error!("data is not ready.");
                            continue;
                        }
                    }
                }

                info!("co2: {}, tvoc: {}", raw.co2(), raw.tvoc());
            }
            Err(e) => log_error(e, &dev).await,
        }
    }

    Ok(())
}
```