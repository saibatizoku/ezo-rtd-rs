//! An example that takes readings from the RTD EZO chip in a loop.
//!

#![recursion_limit = "1024"]

extern crate chrono;
extern crate ezo_rtd;
extern crate i2cdev;

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use ezo_rtd::errors::*;
use ezo_rtd::command::{Command, ReadingWithScale, ScaleKelvin, Sleep};
use ezo_rtd::response::{ResponseStatus, Temperature};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);

    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    let _set_kelvin: ResponseStatus = ScaleKelvin.run(&mut dev)?;

    loop {

        let temperature = ReadingWithScale.run(&mut dev)?;

        let _out = _print_response(temperature)?;

        let _sleep = Sleep.run(&mut dev)?;

        // Ideally, every 10 seconds, fine-tune this to your hardware.
        thread::sleep(Duration::new(9, 300000000));
    }
}

fn _print_response(temp: Temperature) -> Result<()> {
    let dt: DateTime<Utc> = Utc::now();
    println!("{:?},{:?}",
             dt,
             temp,
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
