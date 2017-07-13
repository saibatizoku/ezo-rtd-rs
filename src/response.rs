//! Initial code graciously donated by "Federico Mena Quintero <federico@gnome.org>".

use std::ffi::CStr;
use std::str::FromStr;

use errors::*;

/// Temperature scales supported by the EZO RTD sensor.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TemperatureScale {
    Celsius,
    Kelvin,
    Fahrenheit,
}

impl TemperatureScale {
    /// Parses the result of the "S,?" command to query temperature scale.
    ///
    /// Assumes that the passed response is the device's response without
    /// the initial status byte.
    pub fn parse(response: &[u8]) -> Result<TemperatureScale> {
        let r = str_from_response(response)?;

        match r {
            "?S,c" => Ok(TemperatureScale::Celsius),
            "?S,k" => Ok(TemperatureScale::Kelvin),
            "?S,f" => Ok(TemperatureScale::Fahrenheit),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
    }
}

// Takes in a slice of bytes, and validates that they are nul-terminated and valid UTF-8/ASCII
fn str_from_response(response: &[u8]) -> Result<&str> {
    let terminated = CStr::from_bytes_with_nul(response)
        .chain_err(|| ErrorKind::MalformedResponse)?;
    let r = terminated
        .to_str()
        .chain_err(|| ErrorKind::MalformedResponse)?;

    Ok(r)
}

/// Seconds between automatic logging of readings
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DataLoggerStorageIntervalSeconds(pub u32);

impl DataLoggerStorageIntervalSeconds {
    /// Parses the result of the "D,?" command to query the data logger's
    /// storage interval.  Returns the number of seconds between readings.
    ///
    /// Assumes that the passed response is the device's response without
    /// the initial status byte.
    pub fn parse(response: &[u8]) -> Result<DataLoggerStorageIntervalSeconds> {
        let r = str_from_response(response)?;

        if r.starts_with("?D,") {
            let num_str = r.get(3..).unwrap();
            let num = u32::from_str(num_str)
                .chain_err(|| ErrorKind::ResponseParse)?;
            Ok(DataLoggerStorageIntervalSeconds(num))
        } else {
            Err(ErrorKind::ResponseParse.into())
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
    ///
    /// Assumes that the passed response is the device's response without
    /// the initial status byte.
    pub fn parse(response: &[u8], scale: TemperatureScale) -> Result<Temperature> {
        let r = str_from_response(response)?;
        let val = f64::from_str(r).chain_err(|| ErrorKind::ResponseParse)?;
        Ok(Temperature::new(scale, val))
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
    ///
    /// Assumes that the passed response is the device's response without
    /// the initial status byte.
    pub fn parse(response: &[u8]) -> Result<DeviceStatus> {
        let r = str_from_response(response)?;

        if r.starts_with("?Status,") {
            let rest = r.get(8..).unwrap();
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
    fn parses_temperature_scale() {
        let response = "?S,c\0".as_bytes();
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Celsius);

        let response = "?S,k\0".as_bytes();
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Kelvin);

        let response = "?S,f\0".as_bytes();
        assert_eq!(TemperatureScale::parse(&response).unwrap(),
                   TemperatureScale::Fahrenheit);
    }

    #[test]
    fn parsing_invalid_temperature_scale_yields_error() {
        let response = "".as_bytes();
        assert!(TemperatureScale::parse(&response).is_err());

        let response = "\0".as_bytes();
        assert!(TemperatureScale::parse(&response).is_err());

        let response = "\x01".as_bytes();
        assert!(TemperatureScale::parse(&response).is_err());

        let response = "?S,\0".as_bytes();
        assert!(TemperatureScale::parse(&response).is_err());
    }

    #[test]
    fn parses_data_logger_storage_interval() {
        let response = "?D,1\0".as_bytes();
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(1));

        let response = "?D,42\0".as_bytes();
        assert_eq!(DataLoggerStorageIntervalSeconds::parse(response).unwrap(),
                   DataLoggerStorageIntervalSeconds(42));
    }

    #[test]
    fn parsing_invalid_data_logger_storage_interval_yields_error() {
        let response = "?D,\0".as_bytes();
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,-1\0".as_bytes();
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());

        let response = "?D,foo\0".as_bytes();
        assert!(DataLoggerStorageIntervalSeconds::parse(response).is_err());
    }

    #[test]
    fn parses_temperature() {
        let response = "0\0".as_bytes();
        assert_eq!(Temperature::parse(response, TemperatureScale::Celsius).unwrap(),
                   Temperature::Celsius(0.0));

        let response = "1234.5\0".as_bytes();
        assert_eq!(Temperature::parse(response, TemperatureScale::Kelvin).unwrap(),
                   Temperature::Kelvin(1234.5));

        let response = "-10.5\0".as_bytes();
        assert_eq!(Temperature::parse(response, TemperatureScale::Fahrenheit).unwrap(),
                   Temperature::Fahrenheit(-10.5));
    }

    #[test]
    fn parsing_invalid_temperature_yields_error() {
        let response = "\0".as_bytes();
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());

        let response = "-x\0".as_bytes();
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());
    }

    #[test]
    fn parses_device_status() {
        let response = "?Status,P,1.5\0".as_bytes();
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::PoweredOff,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,S,1.5\0".as_bytes();
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::SoftwareReset,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,B,1.5\0".as_bytes();
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::BrownOut,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,W,1.5\0".as_bytes();
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Watchdog,
                       vcc_voltage: 1.5,
                   });

        let response = "?Status,U,1.5\0".as_bytes();
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Unknown,
                       vcc_voltage: 1.5,
                   });
    }

    #[test]
    fn parsing_invalid_device_status_yields_error() {
        let response = "\0".as_bytes();
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?Status,X,\0".as_bytes();
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?Status,P,1.5,\0".as_bytes();
        assert!(DeviceStatus::parse(response).is_err());
    }
}
