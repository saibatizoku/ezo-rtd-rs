#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate chrono;
extern crate ezo_rtd;
extern crate i2cdev;

use chrono::{DateTime, Utc};
use ezo_rtd::errors::*;
use ezo_rtd::{CommandBuilder, I2cCommand, TemperatureCommand};
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    loop {
        let mut builder = TemperatureCommand::Reading.build();
        builder.run(&mut dev)?;
        let temp = builder.parse_response()?;
        TemperatureCommand::Sleep.build().run(&mut dev)?;
        let dt: DateTime<Utc> = Utc::now();
        println!("{:?},{:.*},°C",
                 dt,
                 2,
                 temp.parse::<f64>().chain_err(|| "unparsable temperature")?);
        thread::sleep(Duration::from_millis(9400));
    }
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}
