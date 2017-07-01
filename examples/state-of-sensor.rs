#![recursion_limit = "1024"]
//! An example that retrieves the current settings of the RTD EZO chip.
//!
extern crate ezo_rtd;
extern crate i2cdev;

use ezo_rtd::errors::*;
use ezo_rtd::{I2cCommand, TemperatureCommand, MAX_RESPONSE_LENGTH};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run_command(dev: &mut LinuxI2CDevice, cmd: TemperatureCommand) -> Result<()> {
    let options = cmd.build();
    let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
    println!("Sending '{:#?}'", options.command);
    dev.write(options.command.as_bytes()).chain_err(|| "Command could not be sent")?;
    if let Some(delay) = options.delay {
        thread::sleep(Duration::from_millis(delay));
    }
    if let Some(response) = options.response {
        dev.read(&mut data_buffer).unwrap();
        match data_buffer[0] {
            255 => println!("No data expected."),
            254 => println!("Pending"),
            2   => println!("Error"),
            1   => {
                println!("Success");
                if let Some(eol) = data_buffer.into_iter().position(|&x| x == 0) {
                    let data: String = data_buffer[1..eol].into_iter().map(|c| {
                        (*c & !0x80) as char
                    }).collect();
                    println!("Response: {}", data);
                } else {
                    println!("Reading: {:?}", String::from_utf8(Vec::from(&data_buffer[1..])).unwrap());
                }
            },
            _ => println!("No response"),
        };
    }
    Ok(())
}

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    run_command(&mut dev, TemperatureCommand::Sleep)?;
    run_command(&mut dev, TemperatureCommand::Status)?;
    run_command(&mut dev, TemperatureCommand::CalibrationState)?;
    run_command(&mut dev, TemperatureCommand::DataloggerInterval)?;
    run_command(&mut dev, TemperatureCommand::LedState)?;
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
