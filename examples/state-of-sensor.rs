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

fn run_command(cmd: TemperatureCommand) -> Result<()> {
    let options = cmd.build();
    let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
    println!("COMMAND: {}", options.command);
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    if let Err(_) = dev.write(options.command.as_bytes()) {
        thread::sleep(Duration::from_millis(300));
        dev.write(options.command.as_bytes())
            .chain_err(|| "Command could not be sent")?;
    };
    if let Some(delay) = options.delay {
        thread::sleep(Duration::from_millis(delay));
    }
    if let Some(_) = options.response {
        dev.read(&mut data_buffer).chain_err(|| "Error reading from device")?;
        match data_buffer[0] {
            255 => println!("No data expected."),
            254 => println!("Pending"),
            2   => println!("Error"),
            1   => {
                let data: String = match data_buffer.into_iter().position(|&x| x == 0) {
                    Some(eol) => {
                        data_buffer[1..eol].into_iter().map(|c| {
                            (*c & !0x80) as char
                        }).collect()
                    },
                    _ => {
                        String::from_utf8(Vec::from(&data_buffer[1..]))
                                .chain_err(|| "Data is not readable")?
                    },
                };
                println!("RESPONSE: {}", data);
            },
            _ => println!("NO RESPONSE"),
        };
    }
    println!();
    Ok(())
}

fn run() -> Result<()> {
    run_command(TemperatureCommand::Status)?;
    run_command(TemperatureCommand::CalibrationState)?;
    run_command(TemperatureCommand::DataloggerInterval)?;
    run_command(TemperatureCommand::LedState)?;
    run_command(TemperatureCommand::Sleep)?;
    run_command(TemperatureCommand::ExportInfo)?;
    run_command(TemperatureCommand::Export)?;
    run_command(TemperatureCommand::Export)?;
    run_command(TemperatureCommand::Export)?;
    run_command(TemperatureCommand::Sleep)?;
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
