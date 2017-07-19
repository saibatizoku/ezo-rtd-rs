//! Initial code graciously donated by "Federico Mena Quintero <federico@gnome.org>".

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
        if response.starts_with("?Cal,") {
            let rest = response.get(5..).unwrap();
            let mut split = rest.split(',');

            match split.next() {
                Some("1") => Ok(CalibrationStatus::Calibrated),
                Some("0") => Ok(CalibrationStatus::NotCalibrated),
                _ => return Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
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
            Ok(DataLoggerStorageIntervalSeconds(num))
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
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
        if response.starts_with("?Export,") {
            let num_str = response.get(8..).unwrap();
            if num_str.starts_with("*") {
                match num_str {
                    "*DONE" => Ok(Exported::Done),
                    _ => Err(ErrorKind::ResponseParse.into()),
                }
            } else {
                match num_str.len() {
                    1..13 => Ok(Exported::ExportString(num_str.to_string())),
                    _ => Err(ErrorKind::ResponseParse.into()),
                }
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
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
        let mut split = response.split(",");

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

        Ok (ExportedInfo { lines, total_bytes } )
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
        if response.starts_with("?i,") {
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

            Ok (DeviceInfo { device, firmware } )

        } else {
            Err(ErrorKind::ResponseParse.into())
        }
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

        Ok (MemoryReading { location, reading })
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
        if response.starts_with("?Plock,") {
            let rest = response.get(7..).unwrap();
            let mut split = rest.split(',');

            match split.next() {
                Some("1") => Ok(ProtocolLockStatus::On),
                Some("0") => Ok(ProtocolLockStatus::Off),
                _ => return Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
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
            "?S,c" => Ok(TemperatureScale::Celsius),
            "?S,k" => Ok(TemperatureScale::Kelvin),
            "?S,f" => Ok(TemperatureScale::Fahrenheit),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
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

    /// Parses the result of the "D" command to get a temperature reading.
    /// Note that this depends on knowing the temperature scale
    /// which the device is configured to use.
    pub fn parse(response: &str, scale: TemperatureScale) -> Result<Temperature> {
        let val = f64::from_str(response).chain_err(|| ErrorKind::ResponseParse)?;
        Ok(Temperature::new(scale, val))
    }
}

/// A temperature reading
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SensorReading(pub f64);

impl SensorReading {
    /// Parses the result of the "T" command to get a temperature reading.
    /// Note that the returned value has no known units. It is your
    /// responsibility to know the current `TemperatureScale` setting.
    pub fn parse(response: &str) -> Result<SensorReading> {
        let val = f64::from_str(response).chain_err(|| ErrorKind::ResponseParse)?;
        Ok(SensorReading(val))
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

/// Response from the "Status" command to get the device status
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DeviceStatus {
    pub restart_reason: RestartReason,
    pub vcc_voltage: f64,
}

impl DeviceStatus {
    /// Parses the result of the "Status" command to get the device's status.
    pub fn parse(response: &str) -> Result<DeviceStatus> {
        if response.starts_with("?Status,") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_calibration_status() {
        let response = "?Cal,1";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::Calibrated);

        let response = "?Cal,0";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::NotCalibrated);
    }

    #[test]
    fn parsing_invalid_calibration_status_yields_error() {
        let response = "";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?Cal,";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?Cal,b";
        assert!(CalibrationStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_data_logger_storage_interval() {
        let response = "?D,1";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(1));

        let response = "?D,42";
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(42));
    }

    #[test]
    fn parsing_invalid_data_logger_storage_interval_yields_error() {
        let response = "?D,";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,-1";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,foo";
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());
    }

    #[test]
    fn parses_data_export_string() {
        let response = "?Export,123456789012";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("123456789012".to_string()));

        let response = "?Export,myresponse";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("myresponse".to_string()));

        let response = "?Export,*DONE";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::Done);
    }

    #[test]
    fn parsing_invalid_export_string_yields_error() {
        let response = "?Export,*DNE";
        assert!(Exported::parse(response).is_err());

        let response = "?Export,*DON";
        assert!(Exported::parse(response).is_err());

        let response = "?Export,**DONE";
        assert!(Exported::parse(response).is_err());

        let response = "?Export,";
        assert!(Exported::parse(response).is_err());

        let response = "?Export,12345678901234567890";
        assert!(Exported::parse(response).is_err());
    }

    #[test]
    fn parses_export_info() {
        let response = "0,0";
        assert_eq!(ExportedInfo::parse(response).unwrap(),
                   ExportedInfo { lines: 0, total_bytes: 0 } );
    }

    #[test]
    fn parsing_invalid_export_info_yields_error() {
        let response = "?10,12";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "10,*DON";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "12,";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "";
        assert!(ExportedInfo::parse(response).is_err());
    }

    #[test]
    fn parses_device_information() {
        let response = "?i,RTD,2.01";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "RTD".to_string(),
                       firmware: "2.01".to_string(),
                   } );

        let response = "?i,,";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "".to_string(),
                       firmware: "".to_string(),
                   } );

    }

    #[test]
    fn parsing_invalid_device_info_yields_error() {
        let response = "?i";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?i,";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "";
        assert!(DeviceInfo::parse(response).is_err());
    }

    #[test]
    fn parses_led_status() {
        let response = "?L,1";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::On);

        let response = "?L,0";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::Off);
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
    fn parses_memory_reading() {
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
    fn parsing_invalid_memory_reading_yields_error() {
        let response = "";
        assert!(MemoryReading::parse(response).is_err());

        let response = "-x";
        assert!(MemoryReading::parse(response).is_err());

        let response = "-1,-1";
        assert!(MemoryReading::parse(response).is_err());
    }

    #[test]
    fn parses_protocol_lock_status() {
        let response = "?Plock,1";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::On);

        let response = "?Plock,0";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::Off);
    }

    #[test]
    fn parsing_invalid_protocol_lock_status_yields_error() {
        let response = "";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?Plock,57";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?Plock,b";
        assert!(ProtocolLockStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_sensor_reading() {
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
    fn parsing_invalid_sensor_reading_yields_error() {
        let response = "";
        assert!(SensorReading::parse(response).is_err());

        let response = "-x";
        assert!(SensorReading::parse(response).is_err());
    }

    #[test]
    fn parses_temperature_scale() {
        let response = "?S,c";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Celsius);

        let response = "?S,k";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Kelvin);

        let response = "?S,f";
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Fahrenheit);
    }

    #[test]
    fn parsing_invalid_temperature_scale_yields_error() {
        let response = "";
        assert!(TemperatureScale::parse(&response).is_err());

        let response = "?S,";
        assert!(TemperatureScale::parse(&response).is_err());
    }

    #[test]
    fn parses_temperature_with_scale() {
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
    fn parsing_invalid_temperature_yields_error() {
        let response = "";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());

        let response = "-x";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());
    }

    #[test]
    fn parses_device_status() {
        let response = "?Status,P,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::PoweredOff,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,S,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::SoftwareReset,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,B,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::BrownOut,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,W,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Watchdog,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,U,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Unknown,
                       vcc_voltage: 1.5,
                   });
    }

    #[test]
    fn parsing_invalid_device_status_yields_error() {
        let response = "";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?Status,X,";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?Status,P,1.5,";
        assert!(DeviceStatus::parse(response).is_err());
    }
}
