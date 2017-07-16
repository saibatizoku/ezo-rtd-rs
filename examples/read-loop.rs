#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate chrono;
extern crate ezo_rtd;
extern crate i2cdev;

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use ezo_rtd::errors::*;
use ezo_rtd::command::{Command, Reading, Sleep};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    loop {
        let _ = Reading.run(&mut dev)?;
        let _ = Sleep.run(&mut dev)?;
        // let _ = _print_response(cmd.parse(), 2, "°C");
        thread::sleep(Duration::from_millis(9400));
    }
}

fn _print_response(response: &str, decimals: usize, units: &str) -> Result<()> {
        let dt: DateTime<Utc> = Utc::now();
        println!("{:?},{:.*},{}",
                 dt,
                 decimals,
                 response.parse::<f64>().chain_err(|| "unparsable temperature")?,
                 units,
                );
        Ok(())
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
