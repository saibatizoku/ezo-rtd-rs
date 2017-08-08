ezo-rtd-rs
==========

Interact with the `RTD EZO` chip, made by Atlas Scientific.

>   Currently, only I2C communication is available.


## Usage

This version needs _nightly_ to compile.

Add this to your `Cargo.toml`:

```
chrono = "0.4.0"
error-chain = "~0.10.0"
ezo_common = { git = "https://github.com/saibatizoku/ezo-common-rs.git", version = "0.1.0" }
ezo_rtd = { git = "https://github.com/saibatizoku/ezo-rtd-rs.git", version = "0.1.0"
i2cdev = "0.3.1"
```

then checkout the examples. :)
