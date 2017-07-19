//! An example that retrieves the current settings of the RTD EZO chip.
//!

#![recursion_limit = "1024"]

#![feature(inclusive_range_syntax)]

extern crate ezo_rtd;
extern crate i2cdev;

use ezo_rtd::errors::*;
use ezo_rtd::command::{
    Command,
    DeviceInformation,
    CalibrationState,
    DataloggerInterval,
    Export,
    ExportInfo,
    LedState,
    ReadingWithScale,
    ScaleCelsius,
    ScaleFahrenheit,
    ScaleKelvin,
    Sleep,
    Status,
};
use ezo_rtd::response::{
    CalibrationStatus,
    DataLoggerStorageIntervalSeconds,
    DeviceInfo,
    DeviceStatus,
    Exported,
    ExportedInfo,
    LedStatus,
};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    let info: DeviceInfo = DeviceInformation.run(&mut dev)?;
    println!("{:?}", info);

    let status: DeviceStatus = Status.run(&mut dev)?;
    println!("DeviceStatus: {:?}", status);

    let calibration: CalibrationStatus = CalibrationState.run(&mut dev)?;
    println!("CalibrationState: {:?}", calibration);

    let datalog_period: DataLoggerStorageIntervalSeconds = DataloggerInterval.run(&mut dev)?;
    println!("{:?}", datalog_period);

    let led_status: LedStatus = LedState.run(&mut dev)?;
    println!("LedState: {:?}", led_status);

    let ExportedInfo { lines, total_bytes } = ExportInfo.run(&mut dev)?;
    println!("ExportInfo: #lines {}, #bytes {}", lines, total_bytes);

    for _ in 0...lines {
        let exports: Exported = Export.run(&mut dev)?;
        println!("Exported: {:?}", exports);
    }

    let _kelvin = ScaleKelvin.run(&mut dev)?;
    println!("Scale set to KELVIN");

    let temperature = ReadingWithScale.run(&mut dev)?;
    println!("{:?}", temperature);

    let _fahrenheit = ScaleFahrenheit.run(&mut dev)?;
    println!("Scale set to FAHRENHEIT");

    let temperature = ReadingWithScale.run(&mut dev)?;
    println!("{:?}", temperature);

    let _celsius = ScaleCelsius.run(&mut dev)?;
    println!("Scale set to CELSIUS");

    let _ = match ReadingWithScale.run(&mut dev) {
        Ok(temperature) => println!("{:?}", temperature),
        Err(e) => {
            match e {
                Error(ErrorKind::PendingResponse, _) => {
                    println!("Response is pending. Try again with a longer delay time.");
                }
                Error(ErrorKind::DeviceErrorResponse, _) => {
                    println!("The device responded with ERR.");
                }
                Error(ErrorKind::NoDataExpectedResponse, _) => {
                    println!("The device responded that it has no data to send.");
                }
                Error(ErrorKind::MalformedResponse, _) => {
                    println!("The device response is unknown.");
                }
                _ => {
                    println!("The response is plainly weird. It should not exist.");
                }
            };
        }
    };

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
