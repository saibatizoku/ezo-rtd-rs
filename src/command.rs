//! I2C commands for the RTD EZO Chip.
//! 
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    DataLoggerStorageIntervalSeconds,
    DeviceStatus,
    Temperature,
    TemperatureScale,
};

use ezo_common::{
    BpsRate,
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_DATA: usize = 16;

/// I2C command for the EZO chip.
pub trait Command {
    type Response;

    fn get_command_string (&self) -> String;
    fn get_delay (&self) -> u64;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Self::Response>;
}

/// `Baud,n` command, where `n` is a variant belonging to `BpsRate`.
define_command! { cmd: Baud(BpsRate), { format!("Baud,{}", cmd.parse() ) }, 0 }

/// `Cal,t` command, where `t` is of type `f64`.
define_command! { cmd: CalibrationTemperature(f64), { format!("Cal,{:.*}", 2, cmd) }, 1000 }

/// `Cal,clear` command.
define_command! { CalibrationClear, { "Cal,clear".to_string() }, 300 }

/// `Cal,?` command.
define_command! { CalibrationState, { "Cal,?".to_string() }, 300 }

/// `Export` command.
define_command! { Export, { "Export".to_string() }, 300 }

/// `ExportInfo` command.
define_command! { ExportInfo, { "Export,?".to_string() }, 300 }

/// `Import,n` command, where `n` is of type `String`.
define_command! { cmd: Import(String), { format!("Import,{}", cmd) }, 300 }

/// `D,n` command, where `n` is of type `u16`.
define_command! { cmd: DataloggerPeriod(u16), { format!("D,{}", cmd) }, 300 }

/// `D,0` command.
define_command! { DataloggerDisable, { "D,0".to_string() }, 300 }

/// `D,?` command. Returns a `DataLoggerStorageIntervalSeconds` response.
define_command! { DataloggerInterval, { "D,?".to_string() }, 300 }

/// `Factory` command.
define_command! { Factory, { "Factory".to_string() }, 0 }

/// `Find` command.
define_command! { Find, { "F".to_string() }, 300 }

/// `I2C,n` command, where `n` is of type `u64`.
define_command! { cmd: DeviceAddress(u16), { format!("I2C,{}", cmd) }, 300 }

/// `I` command.
define_command! { DeviceInformation, { "I".to_string() }, 300 }

/// `L,1` command.
define_command! { LedOn, { "L,1".to_string() }, 300 }

/// `L,0` command.
define_command! { LedOff, { "L,0".to_string() }, 300 }

/// `L,?` command.
define_command! { LedState, { "L,?".to_string() }, 300 }

/// `M,clear` command.
define_command! { MemoryClear, { "M,clear".to_string() }, 300 }

/// `M` command.
define_command! { MemoryRecall, { "M".to_string() }, 300 }

/// `M,?` command.
define_command! { MemoryRecallLast, { "M,?".to_string() }, 300 }

/// `Plock,1` command.
define_command! { ProtocolLockEnable, { "Plock,1".to_string() }, 300 }

/// `Plock,0` command.
define_command! { ProtocolLockDisable, { "Plock,0".to_string() }, 300 }

/// `Plock,?` command.
define_command! { ProtocolLockState, { "Plock,?".to_string() }, 300 }

/// `R` command. Returns a `Temperature` response.
define_command! { Reading, { "R".to_string() }, 600 }

/// `S,c` command.
define_command! { ScaleCelsius, { "S,c".to_string() }, 300 }

/// `S,k` command.
define_command! { ScaleKelvin, { "S,k".to_string() }, 300 }

/// `S,f` command.
define_command! { ScaleFahrenheit, { "S,f".to_string() }, 300 }

/// `S,?` command. Returns a `TemperatureScale` response.
define_command! { ScaleState, { "S,?".to_string() }, 300 }

/// `Status` command. Returns a `DeviceStatus` response.
define_command! { Status, { "Status".to_string() }, 300 }

/// `Sleep` command.
define_command! { Sleep, { "Sleep".to_string() } }

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
