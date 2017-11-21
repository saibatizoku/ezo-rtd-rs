//! I2C commands for the RTD EZO Chip.
//! 
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    DataLoggerStorageIntervalSeconds,
    MemoryReading,
    SensorReading,
    Temperature,
    TemperatureScale,
};

use ezo_common::{
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};
use ezo_common::response::ResponseStatus;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_DATA: usize = 16;

/// I2C command for the EZO chip.
pub use ezo_common::Command;
pub use ezo_common::command::*;


define_command! {
    doc: "`CAL,t` command, where `t` is of type `f64`.",
    arg: CalibrationTemperature(f64), { format!("CAL,{:.*}", 2, arg) }, 1000, Ack
}

impl FromStr for CalibrationTemperature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,") {
            let rest = supper.get(4..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationTemperature(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,?` command. Returns a `CalibrationStatus` response.",
    CalibrationState, { "CAL,?".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

impl FromStr for CalibrationState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,?" => Ok(CalibrationState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`D,n` command, where `n` is of type `u32`, greater than 0.",
    arg: DataloggerPeriod(u32), { format!("D,{}", arg) }, 300, Ack
}

impl FromStr for DataloggerPeriod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("D,") {
            let rest = supper.get(2..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) if n != "0" => {
                    n.parse::<u32>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(DataloggerPeriod(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`D,0` command.",
    DataloggerDisable, { "D,0".to_string() }, 300, Ack
}

impl FromStr for DataloggerDisable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "D,0" => Ok(DataloggerDisable),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`D,?` command. Returns a `DataLoggerStorageIntervalSeconds` response.",
    DataloggerInterval, { "D,?".to_string() }, 300,
    resp: DataLoggerStorageIntervalSeconds, { DataLoggerStorageIntervalSeconds::parse(&resp) }
}
impl FromStr for DataloggerInterval {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "D,?" => Ok(DataloggerInterval),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`M,CLEAR` command.",
    MemoryClear, { "M,CLEAR".to_string() }, 300, Ack
}

impl FromStr for MemoryClear {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "M,CLEAR" => Ok(MemoryClear),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`M` command. Returns a `MemoryReading` response.",
    MemoryRecall, { "M".to_string() }, 300,
    resp: MemoryReading, { MemoryReading::parse(&resp) }
}

impl FromStr for MemoryRecall {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "M" => Ok(MemoryRecall),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`M,?` command. Returns a `MemoryReading` response.",
    MemoryRecallLast, { "M,?".to_string() }, 300,
    resp: MemoryReading, { MemoryReading::parse(&resp) }
}

impl FromStr for MemoryRecallLast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "M,?" => Ok(MemoryRecallLast),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`R` command. Returns a `SensorReading` response.",
    Reading, { "R".to_string() }, 600,
    resp: SensorReading, { SensorReading::parse(&resp) }
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "R" => Ok(Reading),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

/// Obtains a temperature with the current scales.
///
/// It first calls ScaleState::run(..), then returns  Reading::run(..)
pub struct ReadingWithScale;

impl Command for ReadingWithScale {
    type Error = Error;
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
                        string_from_response_data(&data_buffer[1..=len])
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

impl FromStr for ScaleCelsius {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "S,C" => Ok(ScaleCelsius),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`S,K` command.",
    ScaleKelvin, { "S,K".to_string() }, 300, Ack
}

impl FromStr for ScaleKelvin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "S,K" => Ok(ScaleKelvin),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}


define_command! {
    doc: "`S,F` command.",
    ScaleFahrenheit, { "S,F".to_string() }, 300, Ack
}

impl FromStr for ScaleFahrenheit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "S,F" => Ok(ScaleFahrenheit),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! { 
    doc: "`S,?` command. Returns a `TemperatureScale` response.",
    ScaleState, { "S,?".to_string() }, 300,
    resp: TemperatureScale, { TemperatureScale::parse(&resp) }
}

impl FromStr for ScaleState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "S,?" => Ok(ScaleState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_calibration_temperature() {
        let cmd = CalibrationTemperature(35.2459);
        assert_eq!(cmd.get_command_string(), "CAL,35.25");
        assert_eq!(cmd.get_delay(), 1000);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_temperature() {
        let cmd = "cal,0".parse::<CalibrationTemperature>().unwrap();
        assert_eq!(cmd, CalibrationTemperature(0_f64));

        let cmd = "CAL,121.43".parse::<CalibrationTemperature>().unwrap();
        assert_eq!(cmd, CalibrationTemperature(121.43));
    }

    #[test]
    fn parse_invalid_command_calibration_temperature_yields_err() {
        let cmd = "cal,".parse::<CalibrationTemperature>();
        assert!(cmd.is_err());

        let cmd = "CAL,1a21.43".parse::<CalibrationTemperature>();
        assert!(cmd.is_err());
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "CAL,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_state() {
        let cmd = "cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);

        let cmd = "Cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);
    }

    #[test]
    fn build_command_data_logger_period() {
        let cmd = DataloggerPeriod(10);
        assert_eq!(cmd.get_command_string(), "D,10");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_data_logger_period() {
        let cmd = "d,10".parse::<DataloggerPeriod>().unwrap();
        assert_eq!(cmd, DataloggerPeriod(10));

        let cmd = "D,200".parse::<DataloggerPeriod>().unwrap();
        assert_eq!(cmd, DataloggerPeriod(200));
    }

    #[test]
    fn parse_invalid_command_data_logger_period_yields_error() {
        let cmd = "d,".parse::<DataloggerPeriod>();
        assert!(cmd.is_err());

        let cmd = "D,2a0".parse::<DataloggerPeriod>();
        assert!(cmd.is_err());
    }

    #[test]
    fn build_command_data_logger_disable() {
        let cmd = DataloggerDisable;
        assert_eq!(cmd.get_command_string(), "D,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_data_logger_disable() {
        let cmd = "d,0".parse::<DataloggerDisable>().unwrap();
        assert_eq!(cmd, DataloggerDisable);

        let cmd = "D,0".parse::<DataloggerDisable>().unwrap();
        assert_eq!(cmd, DataloggerDisable);
    }

    #[test]
    fn build_command_data_logger_interval() {
        let cmd = DataloggerInterval;
        assert_eq!(cmd.get_command_string(), "D,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_data_logger_interval() {
        let cmd = "d,?".parse::<DataloggerInterval>().unwrap();
        assert_eq!(cmd, DataloggerInterval);

        let cmd = "D,?".parse::<DataloggerInterval>().unwrap();
        assert_eq!(cmd, DataloggerInterval);
    }

    #[test]
    fn build_command_memory_clear() {
        let cmd = MemoryClear;
        assert_eq!(cmd.get_command_string(), "M,CLEAR");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_memory_clear() {
        let cmd = "M,clear".parse::<MemoryClear>().unwrap();
        assert_eq!(cmd, MemoryClear);

        let cmd = "M,CLEAR".parse::<MemoryClear>().unwrap();
        assert_eq!(cmd, MemoryClear);
    }

    #[test]
    fn build_command_memory_recall() {
        let cmd = MemoryRecall;
        assert_eq!(cmd.get_command_string(), "M");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_memory_recall() {
        let cmd = "m".parse::<MemoryRecall>().unwrap();
        assert_eq!(cmd, MemoryRecall);

        let cmd = "M".parse::<MemoryRecall>().unwrap();
        assert_eq!(cmd, MemoryRecall);
    }

    #[test]
    fn build_command_memory_recall_location() {
        let cmd = MemoryRecallLast;
        assert_eq!(cmd.get_command_string(), "M,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_memory_recall_location() {
        let cmd = "m,?".parse::<MemoryRecallLast>().unwrap();
        assert_eq!(cmd, MemoryRecallLast);

        let cmd = "M,?".parse::<MemoryRecallLast>().unwrap();
        assert_eq!(cmd, MemoryRecallLast);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 600);
    }

    #[test]
    fn parse_case_insensitive_command_reading() {
        let cmd = "r".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);

        let cmd = "R".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);
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
    fn parse_case_insensitive_command_scale_celsius() {
        let cmd = "s,c".parse::<ScaleCelsius>().unwrap();
        assert_eq!(cmd, ScaleCelsius);

        let cmd = "S,C".parse::<ScaleCelsius>().unwrap();
        assert_eq!(cmd, ScaleCelsius);
    }

    #[test]
    fn build_command_scale_kelvin() {
        let cmd = ScaleKelvin;
        assert_eq!(cmd.get_command_string(), "S,K");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_scale_kelvin() {
        let cmd = "s,k".parse::<ScaleKelvin>().unwrap();
        assert_eq!(cmd, ScaleKelvin);

        let cmd = "S,K".parse::<ScaleKelvin>().unwrap();
        assert_eq!(cmd, ScaleKelvin);
    }

    #[test]
    fn build_command_scale_fahrenheit() {
        let cmd = ScaleFahrenheit;
        assert_eq!(cmd.get_command_string(), "S,F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_scale_fahrenheit() {
        let cmd = "s,f".parse::<ScaleFahrenheit>().unwrap();
        assert_eq!(cmd, ScaleFahrenheit);

        let cmd = "S,F".parse::<ScaleFahrenheit>().unwrap();
        assert_eq!(cmd, ScaleFahrenheit);
    }

    #[test]
    fn build_command_scale_status() {
        let cmd = ScaleState;
        assert_eq!(cmd.get_command_string(), "S,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_scale_status() {
        let cmd = "s,?".parse::<ScaleState>().unwrap();
        assert_eq!(cmd, ScaleState);

        let cmd = "S,?".parse::<ScaleState>().unwrap();
        assert_eq!(cmd, ScaleState);
    }
}
