//! I2C commands for the RTD EZO Chip.
//! 
use std::thread;
use std::time::Duration;

use {MAX_DATA, LinuxI2CDevice};
use errors::*;
use ezo_common::{
    BpsRate,
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};
use i2cdev::core::I2CDevice;
use response::{
    DataLoggerStorageIntervalSeconds,
    DeviceStatus,
    Temperature,
    TemperatureScale,
};

pub trait Command {
    type Response;

    fn get_command_string (&self) -> String;
    fn get_delay (&self) -> u64;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Self::Response>;
}

/// `Baud,n` command, where `n` is a variant belonging to `BpsRate`.
pub struct Baud(pub BpsRate);

impl Command for Baud {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Cal,t` command, where `t` is of type `f64`.
pub struct CalibrationTemperature(pub f64);

impl Command for CalibrationTemperature {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Cal,clear` command.
pub struct CalibrationClear;

impl Command for CalibrationClear {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Cal,?` command.
pub struct CalibrationState;

impl Command for CalibrationState {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Export` command.
pub struct Export;

impl Command for Export {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `ExportInfo` command.
pub struct ExportInfo;

impl Command for ExportInfo {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Import,n` command, where `n` is of type `String`.
pub struct Import(pub String);

impl Command for Import {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `D,n` command, where `n` is of type `u16`.
pub struct DataloggerPeriod(pub u16);

impl Command for DataloggerPeriod {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `D,0` command.
pub struct DataloggerDisable;

impl Command for DataloggerDisable {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `D,?` command. Returns a `DataLoggerStorageIntervalSeconds` response.
pub struct DataloggerInterval;

impl Command for DataloggerInterval {
    type Response = DataLoggerStorageIntervalSeconds;

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<DataLoggerStorageIntervalSeconds> {
        unimplemented!();
    }
}

/// `Factory` command.
pub struct Factory;

impl Command for Factory {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Find` command.
pub struct Find;

impl Command for Find {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `I2C,n` command, where `n` is of type `u64`.
pub struct DeviceAddress(pub u16);

impl Command for DeviceAddress {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `I` command.
pub struct DeviceInformation;

impl Command for DeviceInformation {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `L,1` command.
pub struct LedOn;

impl Command for LedOn {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `L,0` command.
pub struct LedOff;

impl Command for LedOff {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `L,?` command.
pub struct LedState;

impl Command for LedState {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `M,clear` command.
pub struct MemoryClear;

impl Command for MemoryClear {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `M` command.
pub struct MemoryRecall;

impl Command for MemoryRecall {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `M,?` command.
pub struct MemoryRecallLast;

impl Command for MemoryRecallLast {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Plock,1` command.
pub struct ProtocolLockEnable;

impl Command for ProtocolLockEnable {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Plock,0` command.
pub struct ProtocolLockDisable;

impl Command for ProtocolLockDisable {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `Plock,?` command.
pub struct ProtocolLockState;

impl Command for ProtocolLockState {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `R` command. Returns a `Temperature` response.
pub struct Reading;

impl Command for Reading {
    type Response = Temperature;

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<Temperature> {
        unimplemented!();
    }
}

/// `S,c` command.
pub struct ScaleCelsius;

impl Command for ScaleCelsius {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `S,k` command.
pub struct ScaleKelvin;

impl Command for ScaleKelvin {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `S,f` command.
pub struct ScaleFahrenheit;

impl Command for ScaleFahrenheit {
    type Response = ();

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        unimplemented!();
    }
}

/// `S,?` command. Returns a `TemperatureScale` response.
pub struct ScaleState;

impl Command for ScaleState {
    type Response = TemperatureScale;

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<TemperatureScale> {
        unimplemented!();
    }
}

/// `Status` command. Returns a `DeviceStatus` response.
pub struct Status;

impl Command for Status {
    type Response = DeviceStatus;

    fn get_command_string (&self) -> String { unimplemented!(); }
    fn get_delay (&self) -> u64 { unimplemented!(); }
    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<DeviceStatus> {
        unimplemented!();
    }
}

/// `Sleep` command.
pub struct Sleep;

impl Command for Sleep {
    type Response = ();

    fn get_command_string (&self) -> String { "Sleep".to_string() }

    fn get_delay (&self) -> u64 { 0 }

    fn run (&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        let _ = write_to_ezo(dev, self.get_command_string().as_bytes())
                    .chain_err(|| "Error writing to EZO device.")?;
        let delay = self.get_delay ();
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_uart_300() {
        let cmd = Baud(BpsRate::Bps300);
        assert_eq!(cmd.get_command_string(), "Baud,300");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "Baud,1200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "Baud,2400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "Baud,9600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "Baud,19200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "Baud,38400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "Baud,57600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "Baud,115200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_calibration_temperature() {
        let cmd = CalibrationTemperature(35.2459);
        assert_eq!(cmd.get_command_string(), "Cal,35.25");
        assert_eq!(cmd.get_delay(), 1000);
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "Cal,clear");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "Cal,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_data_logger_period() {
        let cmd = DataloggerPeriod(10);
        assert_eq!(cmd.get_command_string(), "D,10");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_data_logger_disable() {
        let cmd = DataloggerDisable;
        assert_eq!(cmd.get_command_string(), "D,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_data_logger_interval() {
        let cmd = DataloggerInterval;
        assert_eq!(cmd.get_command_string(), "D,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "Export");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "Export,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "Import,ABCDEFGHIJKLMNO");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "Factory");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn;
        assert_eq!(cmd.get_command_string(), "L,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff;
        assert_eq!(cmd.get_command_string(), "L,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState;
        assert_eq!(cmd.get_command_string(), "L,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_memory_clear() {
        let cmd = MemoryClear;
        assert_eq!(cmd.get_command_string(), "M,clear");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_memory_recall() {
        let cmd = MemoryRecall;
        assert_eq!(cmd.get_command_string(), "M");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_memory_recall_location() {
        let cmd = MemoryRecallLast;
        assert_eq!(cmd.get_command_string(), "M,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable;
        assert_eq!(cmd.get_command_string(), "Plock,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "Plock,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "Plock,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn build_command_scale_celsius() {
        let cmd = ScaleCelsius;
        assert_eq!(cmd.get_command_string(), "S,c");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_scale_kelvin() {
        let cmd = ScaleKelvin;
        assert_eq!(cmd.get_command_string(), "S,k");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_scale_fahrenheit() {
        let cmd = ScaleFahrenheit;
        assert_eq!(cmd.get_command_string(), "S,f");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_scale_status() {
        let cmd = ScaleState;
        assert_eq!(cmd.get_command_string(), "S,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep;
        assert_eq!(cmd.get_command_string(), "Sleep");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "Status");
        assert_eq!(cmd.get_delay(), 300);
    }
}
