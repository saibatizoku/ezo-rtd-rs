//! Initial code graciously donated by "Federico Mena Quintero <federico@gnome.org>".
use std::result;
use std::fmt;
use std::str::FromStr;

use errors::ErrorKind;
use failure::{Error, ResultExt};

pub use ezo_common::response::{
    DeviceInfo,
    DeviceStatus,
    Exported,
    ExportedInfo,
    LedStatus,
    ResponseStatus,
    RestartReason,
    ProtocolLockStatus,
};

pub type Result<T> = result::Result<T, Error>;

/// Calibration status of the RTD EZO chip.
#[derive(Copy, Clone, PartialEq)]
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

impl fmt::Debug for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::Calibrated => write!(f, "?CAL,1"),
            CalibrationStatus::NotCalibrated => write!(f, "?CAL,0"),
        }
    }
}

impl fmt::Display for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::Calibrated => write!(f, "calibrated"),
            CalibrationStatus::NotCalibrated => write!(f, "not-calibrated"),
        }
    }
}

/// Seconds between automatic logging of readings
#[derive(Copy, Clone, PartialEq)]
pub struct DataLoggerStorageIntervalSeconds(pub u32);

impl DataLoggerStorageIntervalSeconds {
    /// Parses the result of the "D,?" command to query the data logger's
    /// storage interval.  Returns the number of seconds between readings.
    pub fn parse(response: &str) -> Result<DataLoggerStorageIntervalSeconds> {
        if response.starts_with("?D,") {
            let num_str = response.get(3..).unwrap();
            let num = u32::from_str(num_str)
                .context(ErrorKind::ResponseParse)?;
            match num {
                0 | 10...320_000 => Ok(DataLoggerStorageIntervalSeconds(num)),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for DataLoggerStorageIntervalSeconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?D,{}", self.0)
    }
}

impl fmt::Display for DataLoggerStorageIntervalSeconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A recalled temperature reading from memory.
#[derive(Copy, Clone, PartialEq)]
pub struct MemoryReading {
    pub location: u32,
    pub reading: f64,
}

impl MemoryReading {
    pub fn parse(response: &str) -> Result<MemoryReading> {
        let mut split = response.split(",");

        let location: u32 = if let Some(location_str) = split.next() {
            u32::from_str(location_str)
                .context(ErrorKind::ResponseParse)?
        } else {
            return Err(ErrorKind::ResponseParse.into());
        };

        let reading: f64 = if let Some(reading_str) = split.next() {
            f64::from_str(reading_str)
                .context(ErrorKind::ResponseParse)?
        } else {
            return Err(ErrorKind::ResponseParse.into());
        };

        if let Some(_) = split.next() {
            return Err(ErrorKind::ResponseParse.into());
        }

        Ok (MemoryReading { location, reading })
    }
}

impl fmt::Debug for MemoryReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.location, self.reading)
    }
}

impl fmt::Display for MemoryReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.location, self.reading)
    }
}

/// Temperature scales supported by the RTD EZO sensor.
#[derive(Copy, Clone, PartialEq)]
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

impl fmt::Debug for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            TemperatureScale::Celsius => "?S,C",
            TemperatureScale::Kelvin => "?S,K",
            TemperatureScale::Fahrenheit => "?S,F",
        };
        write!(f, "{}", status)
    }
}

impl fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match *self {
            TemperatureScale::Celsius => "celsius",
            TemperatureScale::Kelvin => "kelvin",
            TemperatureScale::Fahrenheit => "fahrenheit",
        };
        write!(f, "{}", status)
    }
}

/// A temperature value from a temperature reading
#[derive(Copy, Clone, PartialEq)]
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
        let val = f64::from_str(response).context(ErrorKind::ResponseParse)?;
        Ok(Temperature::new(scale, val))
    }
}

impl fmt::Debug for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp, scale) = match *self {
            Temperature::Celsius(t) => (t, "celsius"),
            Temperature::Kelvin(t) => (t, "kelvin"),
            Temperature::Fahrenheit(t) => (t, "fahrenheit"),
        };
        write!(f, "{},{}", temp, scale)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp, scale) = match *self {
            Temperature::Celsius(t) => (t, "celsius"),
            Temperature::Kelvin(t) => (t, "kelvin"),
            Temperature::Fahrenheit(t) => (t, "fahrenheit"),
        };
        write!(f, "{},{}", temp, scale)
    }
}

/// A temperature reading
#[derive(Copy, Clone, PartialEq)]
pub struct SensorReading(pub f64);

impl SensorReading {
    /// Parses the result of the "R" command to get a temperature reading.
    /// Note that the returned value has no known units. It is your
    /// responsibility to know the current `TemperatureScale` setting.
    pub fn parse(response: &str) -> Result<SensorReading> {
        let val = f64::from_str(response).context(ErrorKind::ResponseParse)?;
        Ok(SensorReading(val))
    }
}

impl fmt::Debug for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

impl fmt::Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
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
        assert_eq!(format!("{}", calibration_status), "calibrated");

        let calibration_status = CalibrationStatus::NotCalibrated;
        assert_eq!(format!("{}", calibration_status), "not-calibrated");
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
        assert_eq!(format!("{}", interval), "0");

        let interval = DataLoggerStorageIntervalSeconds(10);
        assert_eq!(format!("{}", interval), "10");

        let interval = DataLoggerStorageIntervalSeconds(42);
        assert_eq!(format!("{}", interval), "42");

        let interval = DataLoggerStorageIntervalSeconds(320000);
        assert_eq!(format!("{}", interval), "320000");

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
        assert_eq!(format!("{}", reading), "0.000");

        let reading = SensorReading(1234.5);
        assert_eq!(format!("{}", reading), "1234.500");

        let reading = SensorReading(-10.035);
        assert_eq!(format!("{}", reading), "-10.035");
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
        assert_eq!(format!("{}", scale), "celsius");

        let scale = TemperatureScale::Kelvin;
        assert_eq!(format!("{}", scale), "kelvin");

        let scale = TemperatureScale::Fahrenheit;
        assert_eq!(format!("{}", scale), "fahrenheit");
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
        assert_eq!(format!("{}", temperature), "0,celsius");

        let temperature = Temperature::Kelvin(1234.5);
        assert_eq!(format!("{}", temperature), "1234.5,kelvin");

        let temperature = Temperature::Fahrenheit(-10.5);
        assert_eq!(format!("{}", temperature), "-10.5,fahrenheit");
    }

    #[test]
    fn parsing_invalid_temperature_yields_error() {
        let response = "";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());

        let response = "-x";
        assert!(Temperature::parse(response, TemperatureScale::Celsius).is_err());
    }
}
