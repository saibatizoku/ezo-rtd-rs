//! Initial code graciously donated by "Federico Mena Quintero <federico@gnome.org>".
use std::fmt;
use std::str::FromStr;

use errors::*;

/// Calibration status of the RTD EZO chip.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CalibrationStatus {
    Calibrated,
    NotCalibrated,
}

impl CalibrationStatus {
    /// Parses the result of the "Cal,?" command to query the device's
    /// calibration status.  Returns ...
    pub fn parse(response: &str) -> Result<CalibrationStatus> {
        if response.starts_with("?CAL,") {
            let rest = response.get(5..).unwrap();
            let mut split = rest.split(',');

            let _calibration = match split.next() {
                Some("1") => Ok(CalibrationStatus::Calibrated),
                Some("0") => Ok(CalibrationStatus::NotCalibrated),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _calibration,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            CalibrationStatus::Calibrated => "1",
            CalibrationStatus::NotCalibrated => "0",
        };
        write!(f, "?CAL,{}", status)
    }
}

/// Seconds between automatic logging of readings
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DataLoggerStorageIntervalSeconds(pub u32);

impl DataLoggerStorageIntervalSeconds {
    /// Parses the result of the "D,?" command to query the data logger's
    /// storage interval.  Returns the number of seconds between readings.
    pub fn parse(response: &str) -> Result<DataLoggerStorageIntervalSeconds> {
        if response.starts_with("?D,") {
            let num_str = response.get(3..).unwrap();
            let num = u32::from_str(num_str)
                .chain_err(|| ErrorKind::ResponseParse)?;
            match num {
                0 | 10...320_000 => Ok(DataLoggerStorageIntervalSeconds(num)),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for DataLoggerStorageIntervalSeconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?D,{}", self.0)
    }
}

/// Exported calibration string of the RTD EZO chip.
#[derive(Debug, Clone, PartialEq)]
pub enum Exported {
    ExportString(String),
    Done,
}

impl Exported {
    pub fn parse(response: &str) -> Result<Exported> {
        if response.starts_with("*") {
            match response {
                "*DONE" => Ok(Exported::Done),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            match response.len() {
                1..13 => Ok(Exported::ExportString(response.to_string())),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        }
    }
}

impl fmt::Display for Exported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let export = match *self {
            Exported::Done => "*DONE",
            Exported::ExportString(ref r) => r,
        };
        write!(f, "{}", export)
    }
}

/// Export the current calibration settings of the RTD EZO chip.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportedInfo {
    pub lines: u16,
    pub total_bytes: u16,
}

impl ExportedInfo {
    pub fn parse(response: &str) -> Result<ExportedInfo> {
        if response.starts_with("?EXPORT,") {
            let num_str = response.get(8..).unwrap();

            let mut split = num_str.split(",");

            let lines = if let Some(lines_str) = split.next() {
                u16::from_str(lines_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let total_bytes = if let Some(totalbytes_str) = split.next() {
                u16::from_str(totalbytes_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok (ExportedInfo { lines, total_bytes } )
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for ExportedInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?EXPORT,{},{}", self.lines, self.total_bytes)
    }
}

/// Current firmware settings of the RTD EZO chip.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    pub device: String,
    pub firmware: String,
}

impl DeviceInfo {
    pub fn parse(response: &str) -> Result<DeviceInfo> {
        if response.starts_with("?I,") {
            let rest = response.get(3..).unwrap();
            let mut split = rest.split(',');

            let device = if let Some(device_str) = split.next() {
                device_str.to_string()
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let firmware = if let Some(firmware_str) = split.next() {
                firmware_str.to_string()
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            if firmware.len() == 0 || device.len() == 0 {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok (DeviceInfo { device, firmware } )

        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?I,{},{}", self.device, self.firmware)
    }
}

/// Status of RTD EZO's LED.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LedStatus {
    Off,
    On,
}

impl LedStatus {
    pub fn parse(response: &str) -> Result<LedStatus> {
        if response.starts_with("?L,") {
            let rest = response.get(3..).unwrap();

            match rest {
                "1" => Ok(LedStatus::On),
                "0" => Ok(LedStatus::Off),
                _ => return Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for LedStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            LedStatus::On => "1",
            LedStatus::Off => "0",
        };
        write!(f, "?L,{}", status)
    }
}

/// A recalled temperature reading from memory.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MemoryReading {
    pub location: u32,
    pub reading: f64,
}

impl MemoryReading {
    pub fn parse(response: &str) -> Result<MemoryReading> {
        let mut split = response.split(",");

        let location: u32 = if let Some(location_str) = split.next() {
            u32::from_str(location_str)
                .chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Err(ErrorKind::ResponseParse.into());
        };

        let reading: f64 = if let Some(reading_str) = split.next() {
            f64::from_str(reading_str)
                .chain_err(|| ErrorKind::ResponseParse)?
        } else {
            return Err(ErrorKind::ResponseParse.into());
        };

        if let Some(_) = split.next() {
            return Err(ErrorKind::ResponseParse.into());
        }

        Ok (MemoryReading { location, reading })
    }
}

impl fmt::Display for MemoryReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.location, self.reading)
    }
}

/// Status of I2C protocol lock.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolLockStatus {
    Off,
    On,
}

impl ProtocolLockStatus {
    pub fn parse(response: &str) -> Result<ProtocolLockStatus> {
        if response.starts_with("?PLOCK,") {
            let rest = response.get(7..).unwrap();
            let mut split = rest.split(',');

            let _plock_status = match split.next() {
                Some("1") => Ok(ProtocolLockStatus::On),
                Some("0") => Ok(ProtocolLockStatus::Off),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _plock_status,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for ProtocolLockStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            ProtocolLockStatus::On => "1",
            ProtocolLockStatus::Off => "0",
        };
        write!(f, "?PLOCK,{}", status)
    }
}

/// Temperature scales supported by the RTD EZO sensor.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TemperatureScale {
    Celsius,
    Kelvin,
    Fahrenheit,
}

impl TemperatureScale {
    /// Parses the result of the "S,?" command to query temperature scale.
    pub fn parse(response: &str) -> Result<TemperatureScale> {
        match response {
            "?S,C" => Ok(TemperatureScale::Celsius),
            "?S,K" => Ok(TemperatureScale::Kelvin),
            "?S,F" => Ok(TemperatureScale::Fahrenheit),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
    }
}

impl fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            TemperatureScale::Celsius => "C",
            TemperatureScale::Kelvin => "K",
            TemperatureScale::Fahrenheit => "F",
        };
        write!(f, "?S,{}", status)
    }
}

/// A temperature value from a temperature reading
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Kelvin(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Creates a new temperature value from a given temperature
    /// `scale`.  Note that this function simply copies the `value`
    /// regardless of the `scale`; it does not validate e.g. that a
    /// Kelvin value is not negative.
    pub fn new(scale: TemperatureScale, value: f64) -> Temperature {
        match scale {
            TemperatureScale::Celsius => Temperature::Celsius(value),
            TemperatureScale::Kelvin => Temperature::Kelvin(value),
            TemperatureScale::Fahrenheit => Temperature::Fahrenheit(value),
        }
    }

    /// Parses the result of the "R" command to get a temperature reading.
    /// Note that this depends on knowing the temperature scale
    /// which the device is configured to use.
    pub fn parse(response: &str, scale: TemperatureScale) -> Result<Temperature> {
        let val = f64::from_str(response).chain_err(|| ErrorKind::ResponseParse)?;
        Ok(Temperature::new(scale, val))
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp, scale) = match *self {
            Temperature::Celsius(t) => (t, "CELSIUS"),
            Temperature::Kelvin(t) => (t, "KELVIN"),
            Temperature::Fahrenheit(t) => (t, "FAHRENHEIT"),
        };
        write!(f, "{},{}", temp, scale)
    }
}

/// A temperature reading
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SensorReading(pub f64);

impl SensorReading {
    /// Parses the result of the "R" command to get a temperature reading.
    /// Note that the returned value has no known units. It is your
    /// responsibility to know the current `TemperatureScale` setting.
    pub fn parse(response: &str) -> Result<SensorReading> {
        let val = f64::from_str(response).chain_err(|| ErrorKind::ResponseParse)?;
        Ok(SensorReading(val))
    }
}

impl fmt::Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Reason for which the device restarted, data sheet pp. 58
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RestartReason {
    PoweredOff,
    SoftwareReset,
    BrownOut,
    Watchdog,
    Unknown,
}

impl fmt::Display for RestartReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            RestartReason::PoweredOff => "P",
            RestartReason::SoftwareReset => "S",
            RestartReason::BrownOut => "B",
            RestartReason::Watchdog => "W",
            RestartReason::Unknown => "U",
        };
        write!(f, "{}", status)
    }
}

/// Response from the "Status" command to get the device status
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DeviceStatus {
    pub restart_reason: RestartReason,
    pub vcc_voltage: f64,
}

impl DeviceStatus {
    /// Parses the result of the "Status" command to get the device's status.
    pub fn parse(response: &str) -> Result<DeviceStatus> {
        if response.starts_with("?STATUS,") {
            let rest = response.get(8..).unwrap();
            let mut split = rest.split(',');

            let restart_reason = match split.next() {
                Some("P") => RestartReason::PoweredOff,
                Some("S") => RestartReason::SoftwareReset,
                Some("B") => RestartReason::BrownOut,
                Some("W") => RestartReason::Watchdog,
                Some("U") => RestartReason::Unknown,
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            let voltage = if let Some(voltage_str) = split.next() {
                f64::from_str(voltage_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok(DeviceStatus {
                   restart_reason: restart_reason,
                   vcc_voltage: voltage,
               })
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?STATUS,{},{}", self.restart_reason, self.vcc_voltage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_response_to_calibration_status() {
        let response = "?CAL,1";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::Calibrated);

        let response = "?CAL,0";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::NotCalibrated);
    }

    #[test]
    fn parses_calibration_status_to_response() {
        let calibration_status = CalibrationStatus::Calibrated;
        assert_eq!(format!("{}", calibration_status), "?CAL,1");

        let calibration_status = CalibrationStatus::NotCalibrated;
        assert_eq!(format!("{}", calibration_status), "?CAL,0");
    }

    #[test]
    fn parsing_invalid_calibration_status_yields_error() {
        let response = "";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,b";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,1,";
        assert!(CalibrationStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_response_to_data_logger_storage_interval() {
        let response = "?D,0";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(0));

        let response = "?D,10";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(10));

        let response = "?D,42";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(42));

        let response = "?D,320000";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(320000));
    }

    #[test]
    fn parses_data_logger_storage_interval_to_response() {
        let interval = DataLoggerStorageIntervalSeconds(0);
        assert_eq!(format!("{}", interval), "?D,0");

        let interval = DataLoggerStorageIntervalSeconds(10);
        assert_eq!(format!("{}", interval), "?D,10");

        let interval = DataLoggerStorageIntervalSeconds(42);
        assert_eq!(format!("{}", interval), "?D,42");

        let interval = DataLoggerStorageIntervalSeconds(320000);
        assert_eq!(format!("{}", interval), "?D,320000");

    }

    #[test]
    fn parsing_invalid_data_logger_storage_interval_yields_error() {
        let response = "?D,";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,-1";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,9";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,320001";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,foo";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_data_export_string() {
        let response = "0";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("0".to_string()));

        let response = "012abc";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("012abc".to_string()));

        let response = "123456abcdef";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("123456abcdef".to_string()));

        let response = "*DONE";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::Done);
    }

    #[test]
    fn parses_data_export_string_to_response() {
        let exported = Exported::ExportString("0".to_string());
        assert_eq!(format!("{}", exported), "0");

        let exported = Exported::ExportString("012abc".to_string());
        assert_eq!(format!("{}", exported), "012abc");

        let exported = Exported::ExportString("123456abcdef".to_string());
        assert_eq!(format!("{}", exported), "123456abcdef");

        let exported = Exported::ExportString("*DONE".to_string());
        assert_eq!(format!("{}", exported), "*DONE");
    }

    #[test]
    fn parsing_invalid_export_string_yields_error() {
        let response = "*";
        assert!(Exported::parse(response).is_err());

        let response = "*DONE*";
        assert!(Exported::parse(response).is_err());

        let response = "**DONE";
        assert!(Exported::parse(response).is_err());

        let response = "12345678901234567890";
        assert!(Exported::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_export_info() {
        let response = "?EXPORT,0,0";
        assert_eq!(ExportedInfo::parse(response).unwrap(),
                   ExportedInfo { lines: 0, total_bytes: 0 } );

        let response = "?EXPORT,10,120";
        assert_eq!(ExportedInfo::parse(response).unwrap(),
                   ExportedInfo { lines: 10, total_bytes: 120 } );
    }

    #[test]
    fn parses_export_info_to_response() {
        let export_info = ExportedInfo { lines: 0, total_bytes: 0 };
        assert_eq!(format!("{}", export_info), "?EXPORT,0,0");

        let export_info = ExportedInfo { lines: 10, total_bytes: 120 };
        assert_eq!(format!("{}", export_info), "?EXPORT,10,120");
    }

    #[test]
    fn parsing_invalid_export_info_yields_error() {
        let response = "?EXPORT,11,120,10";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "?EXPORT,1012";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "10,*DON";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "12,";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "";
        assert!(ExportedInfo::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_device_information() {
        let response = "?I,RTD,2.01";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "RTD".to_string(),
                       firmware: "2.01".to_string(),
                   } );

        let response = "?I,RTD,1.98";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "RTD".to_string(),
                       firmware: "1.98".to_string(),
                   } );
    }

    #[test]
    fn parses_device_information_to_response() {
        let device_info = DeviceInfo {
            device: "RTD".to_string(),
            firmware: "2.01".to_string(),
        };
        assert_eq!(format!("{}", device_info), "?I,RTD,2.01");

        let device_info = DeviceInfo {
            device: "RTD".to_string(),
            firmware: "1.98".to_string(),
        };
        assert_eq!(format!("{}", device_info), "?I,RTD,1.98");
    }

    #[test]
    fn parsing_invalid_device_info_yields_error() {
        let response = "";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,,";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,a,b,c";
        assert!(DeviceInfo::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_led_status() {
        let response = "?L,1";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::On);

        let response = "?L,0";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::Off);
    }

    #[test]
    fn parses_led_status_to_response() {
        let led = LedStatus::On;
        assert_eq!(format!("{}", led), "?L,1");

        let led = LedStatus::Off;
        assert_eq!(format!("{}", led), "?L,0");
    }

    #[test]
    fn parsing_invalid_led_status_yields_error() {
        let response = "";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,b";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,17";
        assert!(LedStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_response_to_memory_reading() {
        let response = "0,0";
        assert_eq!(MemoryReading::parse(response).unwrap(),
                   MemoryReading { location: 0, reading: 0.0 });

        let response = "50,1234.5";
        assert_eq!(MemoryReading::parse(response).unwrap(),
                   MemoryReading { location: 50, reading: 1234.5 });

        let response = "17,-10.5";
        assert_eq!(MemoryReading::parse(response).unwrap(),
                   MemoryReading { location: 17, reading: -10.5 });
    }

    #[test]
    fn parses_memory_reading_to_response() {
        let memory = MemoryReading { location: 0, reading: 0.0 };
        assert_eq!(format!("{}", memory), "0,0");

        let memory = MemoryReading { location: 50, reading: 1234.5 };
        assert_eq!(format!("{}", memory), "50,1234.5");

        let memory = MemoryReading { location: 17, reading: -10.5 };
        assert_eq!(format!("{}", memory), "17,-10.5");
    }

    #[test]
    fn parsing_invalid_memory_reading_yields_error() {
        let response = "";
        assert!(MemoryReading::parse(response).is_err());

        let response = "-x";
        assert!(MemoryReading::parse(response).is_err());

        let response = "-1,-1";
        assert!(MemoryReading::parse(response).is_err());

        let response = "1,1,1";
        assert!(MemoryReading::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_protocol_lock_status() {
        let response = "?PLOCK,1";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::On);

        let response = "?PLOCK,0";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::Off);
    }

    #[test]
    fn parses_protocol_lock_status_to_response() {
        let plock = ProtocolLockStatus::On;
        assert_eq!(format!("{}", plock), "?PLOCK,1");

        let plock = ProtocolLockStatus::Off;
        assert_eq!(format!("{}", plock), "?PLOCK,0");
    }

    #[test]
    fn parsing_invalid_protocol_lock_status_yields_error() {
        let response = "";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,57";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b,1";
        assert!(ProtocolLockStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_response_to_sensor_reading() {
        let response = "0";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(0.0));

        let response = "1234.5";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(1234.5));

        let response = "-10.5";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(-10.5));
    }

    #[test]
    fn parses_sensor_reading_to_response() {
        let reading = SensorReading(0.0);
        assert_eq!(format!("{}", reading), "0");

        let reading = SensorReading(1234.5);
        assert_eq!(format!("{}", reading), "1234.5");

        let reading = SensorReading(-10.5);
        assert_eq!(format!("{}", reading), "-10.5");
    }

    #[test]
    fn parsing_invalid_sensor_reading_yields_error() {
        let response = "";
        assert!(SensorReading::parse(response).is_err());

        let response = "-x";
        assert!(SensorReading::parse(response).is_err());
    }

    #[test]
    fn parses_response_to_temperature_scale() {
        let response = "?S,C";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Celsius);

        let response = "?S,K";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Kelvin);

        let response = "?S,F";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Fahrenheit);
    }

    #[test]
    fn parses_temperature_scale_to_response() {
        let scale = TemperatureScale::Celsius;
        assert_eq!(format!("{}", scale), "?S,C");

        let scale = TemperatureScale::Kelvin;
        assert_eq!(format!("{}", scale), "?S,K");

        let scale = TemperatureScale::Fahrenheit;
        assert_eq!(format!("{}", scale), "?S,F");
    }

    #[test]
    fn parsing_invalid_temperature_scale_yields_error() {
        let response = "";
        assert!(TemperatureScale::parse(&response).is_err());

        let response = "?S,";
        assert!(TemperatureScale::parse(&response).is_err());
    }

    #[test]
    fn parses_response_to_temperature_with_scale() {
        let response = "0";
        assert_eq!(Temperature::parse(response, TemperatureScale::Celsius).unwrap(),
                   Temperature::Celsius(0.0));

        let response = "1234.5";
        assert_eq!(Temperature::parse(response, TemperatureScale::Kelvin).unwrap(),
                   Temperature::Kelvin(1234.5));

        let response = "-10.5";
        assert_eq!(Temperature::parse(response, TemperatureScale::Fahrenheit).unwrap(),
                   Temperature::Fahrenheit(-10.5));
    }

    #[test]
    fn parses_temperature_with_scale_to_response() {
        let temperature = Temperature::Celsius(0.0);
        assert_eq!(format!("{}", temperature), "0,CELSIUS");

        let temperature = Temperature::Kelvin(1234.5);
        assert_eq!(format!("{}", temperature), "1234.5,KELVIN");

        let temperature = Temperature::Fahrenheit(-10.5);
        assert_eq!(format!("{}", temperature), "-10.5,FAHRENHEIT");
    }

    #[test]
    fn parsing_invalid_temperature_yields_error() {
        let response = "";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());

        let response = "-x";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());
    }

    #[test]
    fn parses_response_to_device_status() {
        let response = "?STATUS,P,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::PoweredOff,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,S,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::SoftwareReset,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,B,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::BrownOut,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,W,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Watchdog,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,U,1.5";
        let device_status = DeviceStatus {
            restart_reason: RestartReason::Unknown,
            vcc_voltage: 1.5,
        };
        assert_eq!(DeviceStatus::parse(response).unwrap(), device_status);
    }

    #[test]
    fn parses_device_status_to_response() {
        let device_status = DeviceStatus {
            restart_reason: RestartReason::Unknown,
            vcc_voltage: 3.15,
        };
        assert_eq!(format!("{}", device_status), "?STATUS,U,3.15");
    }

    #[test]
    fn parsing_invalid_device_status_yields_error() {
        let response = "";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?STATUS,X,";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?Status,P,1.5,";
        assert!(DeviceStatus::parse(response).is_err());
    }
}
