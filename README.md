# ccs811

This is to use CCS811 with Rust.

[SparkFun Air Quality Breakout \- CCS811 \- SEN\-14193 \- SparkFun Electronics](https://www.sparkfun.com/products/14193)


```toml
ccs811 = { git = "https://github.com/mmmpa/rs_ccs811", features = ["std"] }
```

```rust
use ccs811::unix::i2c::{I2cAddress, I2cBus};
use ccs811::unix::Ccs811Client;
use ccs811::{Ccs811, MeasureDriveMode, MeasureInterrupt, MeasureThresh};

fn bus_number() -> I2cBus {
    I2cBus(env!("I2C_DEVICE_NUMBER").parse().unwrap())
}

fn device_address() -> I2cAddress {
    let no_prefix = env!("I2C_DEVICE_ADDRESS").trim_start_matches("0x");
    I2cAddress(u8::from_str_radix(no_prefix, 16).unwrap())
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let dev = Ccs811Client::new(bus_number(), device_address())?;

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
                        log_error(e, &dev).await;
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