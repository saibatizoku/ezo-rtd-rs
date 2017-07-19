//! An example that retrieves the current settings of the RTD EZO chip.
//!

#![recursion_limit = "1024"]

#![feature(inclusive_range_syntax)]

extern crate ezo_rtd;
extern crate i2cdev;

use ezo_rtd::errors::*;
use ezo_rtd::command::{
    Command,
    Status,
    CalibrationState,
    DataloggerInterval,
    LedState,
    Export,
    ExportInfo,
    Sleep,
};
use ezo_rtd::response::{
    DeviceStatus,
    CalibrationStatus,
    DataLoggerStorageIntervalSeconds,
    LedStatus,
    Exported,
    ExportedInfo,
};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    let status: DeviceStatus = Status.run(&mut dev)?;
    println!("DeviceStatus: {:?}", status);

    let calibration: CalibrationStatus = CalibrationState.run(&mut dev)?;
    println!("CalibrationState: {:?}", calibration);

    let datalog_period: DataLoggerStorageIntervalSeconds = DataloggerInterval.run(&mut dev)?;
    println!("DataloggerInterval: {:#?}", datalog_period);

    let led_status: LedStatus = LedState.run(&mut dev)?;
    println!("LedState: {:#?}", led_status);

    let ExportedInfo { lines, total_bytes } = ExportInfo.run(&mut dev)?;
    println!("ExportInfo: #lines {:#?}, #bytes {:#?}", lines, total_bytes);

    for _ in 0...lines {
        let exports: Exported = Export.run(&mut dev)?;
        println!("Exported: {:#?}", exports);
    }

    let _ = match Sleep.run(&mut dev) {
        Err(_) => println!("Sleeping...."),
        _ => (),
    };

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
