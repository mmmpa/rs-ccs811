# ccs811

This is to use CCS811 with Rust.

[SparkFun Air Quality Breakout \- CCS811 \- SEN\-14193 \- SparkFun Electronics](https://www.sparkfun.com/products/14193)


# example

## env

```sh
export I2C_DEVICE_PATH="/dev/i2c-1"
export I2C_DEVICE_ADDRESS="0x5b"
```

## .cargo/config

```toml
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

## build

```shell
cargo build --example printer --target arm-unknown-linux-gnueabi --release --features "std,with_tokio"
```
