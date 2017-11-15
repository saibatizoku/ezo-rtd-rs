//! I2C commands for the RTD EZO Chip.
//! 
use std::str::FromStr;
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

impl FromStr for Baud {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("BAUD,") {
            let rest = supper.get(5..).unwrap();
            let mut split = rest.split(',');
            let rate = match split.next() {
                Some("300") => BpsRate::Bps300,
                Some("1200") => BpsRate::Bps1200,
                Some("2400") => BpsRate::Bps2400,
                Some("9600") => BpsRate::Bps9600,
                Some("19200") => BpsRate::Bps19200,
                Some("38400") => BpsRate::Bps38400,
                Some("57600") => BpsRate::Bps57600,
                Some("115200") => BpsRate::Bps115200,
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(Baud(rate)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

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
    doc: "`CAL,CLEAR` command.",
    CalibrationClear, { "CAL,CLEAR".to_string() }, 300, Ack
}

impl FromStr for CalibrationClear {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,CLEAR" => Ok(CalibrationClear),
            _ => bail!(ErrorKind::CommandParse),
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
    doc: "`EXPORT` command.",
    Export, { "EXPORT".to_string() }, 300,
    resp: Exported, { Exported::parse(&resp) }
}

impl FromStr for Export {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "EXPORT" => Ok(Export),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`EXPORT,?` command.",
    ExportInfo, { "EXPORT,?".to_string() }, 300,
    resp: ExportedInfo, { ExportedInfo::parse(&resp) }
}

impl FromStr for ExportInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "EXPORT,?" => Ok(ExportInfo),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`IMPORT,n` command, where `n` is of type `String`.",
    arg: Import(String), { format!("IMPORT,{}", arg) }, 300, Ack
}

impl FromStr for Import {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("IMPORT,") {
            let rest = supper.get(7..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) if n.len() > 0 && n.len() < 13 => {
                    n.to_string()
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(Import(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
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
    doc: "`FACTORY` command.",
    Factory, { "FACTORY".to_string() }, 0
}
impl FromStr for Factory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "FACTORY" => Ok(Factory),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}


define_command! {
    doc: "`F`ind command.",
    Find, { "F".to_string() }, 300
}

impl FromStr for Find {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "F" => Ok(Find),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`I2C,n` command, where `n` is of type `u16`.",
    arg: DeviceAddress(u16), { format!("I2C,{}", arg) }, 300
}

impl FromStr for DeviceAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("I2C,") {
            let rest = supper.get(4..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<u16>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(DeviceAddress(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`I` command.",
    DeviceInformation, { "I".to_string() }, 300,
    resp: DeviceInfo, { DeviceInfo::parse(&resp) }
}

impl FromStr for DeviceInformation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "I" => Ok(DeviceInformation),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,1` command.",
    LedOn, { "L,1".to_string() }, 300, Ack
}

impl FromStr for LedOn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,1" => Ok(LedOn),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,0` command.",
    LedOff, { "L,0".to_string() }, 300, Ack
}

impl FromStr for LedOff {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,0" => Ok(LedOff),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`L,?` command.",
    LedState, { "L,?".to_string() }, 300,
    resp: LedStatus, { LedStatus::parse(&resp) }
}

impl FromStr for LedState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "L,?" => Ok(LedState),
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
    doc: "`PLOCK,1` command.",
    ProtocolLockEnable, { "PLOCK,1".to_string() }, 300, Ack
}

impl FromStr for ProtocolLockEnable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,1" => Ok(ProtocolLockEnable),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`PLOCK,0` command.",
    ProtocolLockDisable, { "PLOCK,0".to_string() }, 300, Ack
}

impl FromStr for ProtocolLockDisable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,0" => Ok(ProtocolLockDisable),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`PLOCK,?` command. Returns a `ProtocolLockStatus` response.",
    ProtocolLockState, { "PLOCK,?".to_string() }, 300,
    resp: ProtocolLockStatus, { ProtocolLockStatus::parse(&resp) }
}

impl FromStr for ProtocolLockState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "PLOCK,?" => Ok(ProtocolLockState),
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


define_command! { 
    doc: "`STATUS` command. Returns a `DeviceStatus` response.",
    Status, { "STATUS".to_string() }, 300,
    resp: DeviceStatus, { DeviceStatus::parse(&resp) }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "STATUS" => Ok(Status),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`SLEEP` command.",
    Sleep, { "SLEEP".to_string() }, 0
}

impl FromStr for Sleep {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "SLEEP" => Ok(Sleep),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
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
    fn parse_case_insensitive_command_baud_300() {
        let cmd = "baud,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));

        let cmd = "BAUD,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));
    }

    #[test]
    fn build_command_uart_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "BAUD,1200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_1200() {
        let cmd = "baud,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));

        let cmd = "BAUD,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));
    }

    #[test]
    fn build_command_uart_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "BAUD,2400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_2400() {
        let cmd = "baud,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));

        let cmd = "BAUD,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));
    }

    #[test]
    fn build_command_uart_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "BAUD,9600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_9600() {
        let cmd = "baud,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));

        let cmd = "BAUD,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));
    }

    #[test]
    fn build_command_uart_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "BAUD,19200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_19200() {
        let cmd = "baud,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));

        let cmd = "BAUD,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));
    }

    #[test]
    fn build_command_uart_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "BAUD,38400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_38400() {
        let cmd = "baud,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));

        let cmd = "BAUD,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));
    }

    #[test]
    fn build_command_uart_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "BAUD,57600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_57600() {
        let cmd = "baud,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));

        let cmd = "BAUD,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));
    }

    #[test]
    fn build_command_uart_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "BAUD,115200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_115200() {
        let cmd = "baud,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));

        let cmd = "BAUD,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));
    }

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
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "CAL,CLEAR");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_clear() {
        let cmd = "cal,clear".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);

        let cmd = "Cal,CLEAR".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);
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
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "EXPORT");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export() {
        let cmd = "export".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);

        let cmd = "EXPORT".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "EXPORT,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export_info() {
        let cmd = "export,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);

        let cmd = "EXPORT,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "IMPORT,ABCDEFGHIJKLMNO");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_import() {
        let cmd = "import,1".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("1".to_string()));

        let cmd = "IMPORT,abcdef".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("ABCDEF".to_string()));
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "FACTORY");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_factory() {
        let cmd = "factory".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);

        let cmd = "FACTORY".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_find() {
        let cmd = "f".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);

        let cmd = "F".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_information() {
        let cmd = "i".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);

        let cmd = "I".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_command_insensitive_device_address() {
        let cmd = "i2c,1".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(1));

        let cmd = "I2C,123".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(123));
    }

    #[test]
    fn parse_invalid_command_device_address_yields_error() {
        let cmd = "I2C,".parse::<DeviceAddress>();
        assert!(cmd.is_err());

        let cmd = "I2C,a".parse::<DeviceAddress>();
        assert!(cmd.is_err());

        let cmd = "I2C,2a0".parse::<DeviceAddress>();
        assert!(cmd.is_err());
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn;
        assert_eq!(cmd.get_command_string(), "L,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_on() {
        let cmd = "l,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);

        let cmd = "L,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff;
        assert_eq!(cmd.get_command_string(), "L,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_off() {
        let cmd = "l,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);

        let cmd = "L,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState;
        assert_eq!(cmd.get_command_string(), "L,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_state() {
        let cmd = "l,?".parse::<LedState>().unwrap();
        assert_eq!(cmd, LedState);

        let cmd = "L,?".parse::<LedState>().unwrap();
        assert_eq!(cmd, LedState);
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
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable;
        assert_eq!(cmd.get_command_string(), "PLOCK,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_enable() {
        let cmd = "plock,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);

        let cmd = "PLOCK,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "PLOCK,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_disable() {
        let cmd = "plock,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);

        let cmd = "PLOCK,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "PLOCK,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_status() {
        let cmd = "plock,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);

        let cmd = "PLOCK,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);
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

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep;
        assert_eq!(cmd.get_command_string(), "SLEEP");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_sleep() {
        let cmd = "Sleep".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);

        let cmd = "SLEEP".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "STATUS");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_status() {
        let cmd = "status".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);

        let cmd = "STATUS".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);
    }
}
