//! I2C commands for the RTD EZO Chip.
//! 
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    DataLoggerStorageIntervalSeconds,
    Exported,
    ExportedInfo,
    DeviceInfo,
    LedStatus,
    MemoryReading,
    ProtocolLockStatus,
    SensorReading,
    Temperature,
    TemperatureScale,
    DeviceStatus,
};

use ezo_common::{
    BpsRate,
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};
use i2cdev::core::I2CDevice;
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

define_command! {
    doc: "`BAUD,n` command, where `n` is a variant belonging to `BpsRate`.",
    arg: Baud(BpsRate), { format!("BAUD,{}", arg.parse() ) }, 0
}

define_command! {
    doc: "`CAL,t` command, where `t` is of type `f64`.",
    arg: CalibrationTemperature(f64), { format!("CAL,{:.*}", 2, arg) }, 1000, Ack
}

define_command! {
    doc: "`CAL,CLEAR` command.",
    CalibrationClear, { "CAL,CLEAR".to_string() }, 300, Ack
}

define_command! {
    doc: "`CAL,?` command. Returns a `CalibrationStatus` response.",
    CalibrationState, { "CAL,?".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

define_command! {
    doc: "`EXPORT` command.",
    Export, { "EXPORT".to_string() }, 300,
    resp: Exported, { Exported::parse(&resp) }
}

define_command! {
    doc: "`EXPORT,?` command.",
    ExportInfo, { "EXPORT,?".to_string() }, 300,
    resp: ExportedInfo, { ExportedInfo::parse(&resp) }
}

define_command! {
    doc: "`IMPORT,n` command, where `n` is of type `String`.",
    arg: Import(String), { format!("IMPORT,{}", arg) }, 300, Ack
}

define_command! {
    doc: "`D,n` command, where `n` is of type `u16`.",
    arg: DataloggerPeriod(u16), { format!("D,{}", arg) }, 300, Ack
}

define_command! {
    doc: "`D,0` command.",
    DataloggerDisable, { "D,0".to_string() }, 300, Ack
}

define_command! {
    doc: "`D,?` command. Returns a `DataLoggerStorageIntervalSeconds` response.",
    DataloggerInterval, { "D,?".to_string() }, 300,
    resp: DataLoggerStorageIntervalSeconds, { DataLoggerStorageIntervalSeconds::parse(&resp) }
}

define_command! {
    doc: "`FACTORY` command.",
    Factory, { "FACTORY".to_string() }, 0
}

define_command! {
    doc: "`F`ind command.",
    Find, { "F".to_string() }, 300
}

define_command! {
    doc: "`I2C,n` command, where `n` is of type `u64`.",
    arg: DeviceAddress(u16), { format!("I2C,{}", arg) }, 300
}

define_command! {
    doc: "`I` command.",
    DeviceInformation, { "I".to_string() }, 300,
    resp: DeviceInfo, { DeviceInfo::parse(&resp) }
}

define_command! {
    doc: "`L,1` command.",
    LedOn, { "L,1".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,0` command.",
    LedOff, { "L,0".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,?` command.",
    LedState, { "L,?".to_string() }, 300,
    resp: LedStatus, { LedStatus::parse(&resp) }
}

define_command! {
    doc: "`M,CLEAR` command.",
    MemoryClear, { "M,CLEAR".to_string() }, 300, Ack
}

define_command! {
    doc: "`M` command. Returns a `MemoryReading` response.",
    MemoryRecall, { "M".to_string() }, 300,
    resp: MemoryReading, { MemoryReading::parse(&resp) }
}

define_command! {
    doc: "`M,?` command. Returns a `MemoryReading` response.",
    MemoryRecallLast, { "M,?".to_string() }, 300,
    resp: MemoryReading, { MemoryReading::parse(&resp) }
}

define_command! {
    doc: "`PLOCK,1` command.",
    ProtocolLockEnable, { "PLOCK,1".to_string() }, 300, Ack
}

define_command! {
    doc: "`PLOCK,0` command.",
    ProtocolLockDisable, { "PLOCK,0".to_string() }, 300, Ack
}

define_command! {
    doc: "`PLOCK,?` command. Returns a `ProtocolLockStatus` response.",
    ProtocolLockState, { "PLOCK,?".to_string() }, 300,
    resp: ProtocolLockStatus, { ProtocolLockStatus::parse(&resp) }
}

define_command! {
    doc: "`R` command. Returns a `SensorReading` response.",
    Reading, { "R".to_string() }, 600,
    resp: SensorReading, { SensorReading::parse(&resp) }
}

/// Obtains a temperature with the current scales.
///
/// It first calls ScaleState::run(..), then returns  Reading::run(..)
pub struct ReadingWithScale;

impl Command for ReadingWithScale {
    type Response = Temperature;

    fn get_command_string(&self) -> String {
        Reading.get_command_string()
    }

    fn get_delay(&self) -> u64 {
        // This command involves the sequential execution of
        // `ScaleState.run(..)` and `Reading.run(..)`, thus
        // the resulting delay is the sum of both commands.
        ScaleState.get_delay() + Reading.get_delay()
    }

    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Temperature> {

        let scale = ScaleState.run(dev)?;

        let cmd = Reading.get_command_string();

        let _w = write_to_ezo(dev, &cmd)
            .chain_err(|| "Error writing to EZO device.")?;

        let _wait = thread::sleep(Duration::from_millis(Reading.get_delay()));

        let mut data_buffer = [0u8; MAX_DATA];

        let _r = dev.read(&mut data_buffer)
            .chain_err(|| ErrorKind::I2CRead)?;

        let resp_string = match response_code(data_buffer[0]) {

            ResponseCode::Success => {
                match data_buffer.iter().position(|&c| c == 0) {
                    Some(len) => {
                        string_from_response_data(&data_buffer[1...len])
                            .chain_err(|| ErrorKind::MalformedResponse)
                    }
                    _ => return Err(ErrorKind::MalformedResponse.into()),
                }
            }

            ResponseCode::Pending => return Err(ErrorKind::PendingResponse.into()),

            ResponseCode::DeviceError => return Err(ErrorKind::DeviceErrorResponse.into()),

            ResponseCode::NoDataExpected => return Err(ErrorKind::NoDataExpectedResponse.into()),

            ResponseCode::UnknownError => return Err(ErrorKind::MalformedResponse.into()),
        };

        Temperature::parse(&resp_string?, scale)
    }
}

define_command! {
    doc: "`S,C` command.",
    ScaleCelsius, { "S,C".to_string() }, 300, Ack
}

define_command! {
    doc: "`S,K` command.",
    ScaleKelvin, { "S,K".to_string() }, 300, Ack
}


define_command! {
    doc: "`S,F` command.",
    ScaleFahrenheit, { "S,F".to_string() }, 300, Ack
}


define_command! { 
    doc: "`S,?` command. Returns a `TemperatureScale` response.",
    ScaleState, { "S,?".to_string() }, 300,
    resp: TemperatureScale, { TemperatureScale::parse(&resp) }
}


define_command! { 
    doc: "`STATUS` command. Returns a `DeviceStatus` response.",
    Status, { "STATUS".to_string() }, 300,
    resp: DeviceStatus, { DeviceStatus::parse(&resp) }
}

define_command! {
    doc: "`SLEEP` command.",
    Sleep, { "SLEEP".to_string() }, 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_uart_300() {
        let cmd = Baud(BpsRate::Bps300);
        assert_eq!(cmd.get_command_string(), "BAUD,300");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "BAUD,1200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "BAUD,2400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "BAUD,9600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "BAUD,19200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "BAUD,38400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "BAUD,57600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "BAUD,115200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_calibration_temperature() {
        let cmd = CalibrationTemperature(35.2459);
        assert_eq!(cmd.get_command_string(), "CAL,35.25");
        assert_eq!(cmd.get_delay(), 1000);
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "CAL,CLEAR");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "CAL,?");
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
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "EXPORT");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "EXPORT,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "IMPORT,ABCDEFGHIJKLMNO");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "FACTORY");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88");
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
        assert_eq!(cmd.get_command_string(), "M,CLEAR");
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
        assert_eq!(cmd.get_command_string(), "PLOCK,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "PLOCK,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "PLOCK,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn build_command_reading_with_scale() {
        let cmd = ReadingWithScale;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn build_command_scale_celsius() {
        let cmd = ScaleCelsius;
        assert_eq!(cmd.get_command_string(), "S,C");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_scale_kelvin() {
        let cmd = ScaleKelvin;
        assert_eq!(cmd.get_command_string(), "S,K");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_scale_fahrenheit() {
        let cmd = ScaleFahrenheit;
        assert_eq!(cmd.get_command_string(), "S,F");
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
        assert_eq!(cmd.get_command_string(), "SLEEP");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "STATUS");
        assert_eq!(cmd.get_delay(), 300);
    }
}
