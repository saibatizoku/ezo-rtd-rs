//! An example that retrieves the current settings of the RTD EZO chip.
//!
extern crate ezo_rtd;
extern crate failure;
extern crate i2cdev;

use ezo_rtd::command::{Command, DeviceInformation, CalibrationState, DataloggerInterval, Export,
                       ExportInfo, LedState, ReadingWithScale, ScaleCelsius, ScaleFahrenheit,
                       ScaleKelvin, Sleep, Status};
use ezo_rtd::response::{CalibrationStatus, DataLoggerStorageIntervalSeconds, DeviceInfo,
                        DeviceStatus, Exported, ExportedInfo, LedStatus};
use failure::{Error, ResultExt};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65

fn run() -> Result<(), Error> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .context("Could not open I2C device")?;

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

    for _ in 0..=lines {
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

    let _reading = match ReadingWithScale.run(&mut dev) {

        Ok(temperature) => println!("{:?}", temperature),

        Err(e) => {
            println!("Error: {}", e);
        }
    };

    let _sleep = Sleep.run(&mut dev)?;
    println!("Sleeping....");

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        let backtrace = e.backtrace();
        println!("backtrace: {:?}", backtrace);
        ::std::process::exit(1);
    }
}
