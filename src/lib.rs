//! I2C Commands for RTD EZO Chip, taken from their Datasheet.
//! This chip is used for temperature measurement. It features
//! calibration, sleep mode, scale, etc.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#![feature(exclusive_range_pattern)]

#![feature(inclusive_range_syntax)]

#![feature(trace_macros)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate ezo_common;
extern crate i2cdev;
extern crate lalrpop_util;

// Use error-chain.
pub mod errors;

/// Issuable commands for the EZO RTD Chip.
pub mod command;

/// Parseable responses from the EZO RTD Chip.
pub mod response;

/// Parser for the RTD EZO chip.
pub mod parsers;


#[cfg(test)]
mod tests {
    use super::*;
    use ezo_common::BpsRate;

    #[test]
    fn parses_baud_rates() {
        let cmd = parsers::parse_NumBaud("300").unwrap();
        assert_eq!(cmd, BpsRate::Bps300);

        let cmd = parsers::parse_NumBaud("1200").unwrap();
        assert_eq!(cmd, BpsRate::Bps1200);

        let cmd = parsers::parse_NumBaud("2400").unwrap();
        assert_eq!(cmd, BpsRate::Bps2400);

        let cmd = parsers::parse_NumBaud("9600").unwrap();
        assert_eq!(cmd, BpsRate::Bps9600);

        let cmd = parsers::parse_NumBaud("19200").unwrap();
        assert_eq!(cmd, BpsRate::Bps19200);

        let cmd = parsers::parse_NumBaud("38400").unwrap();
        assert_eq!(cmd, BpsRate::Bps38400);

        let cmd = parsers::parse_NumBaud("57600").unwrap();
        assert_eq!(cmd, BpsRate::Bps57600);

        let cmd = parsers::parse_NumBaud("115200").unwrap();
        assert_eq!(cmd, BpsRate::Bps115200);
    }

    #[test]
    fn parsing_invalid_baud_rates_yields_err() {
        let cmd = parsers::parse_NumBaud("8600");
        assert!(cmd.is_err());

        let cmd = parsers::parse_NumBaud("");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_baud_commands() {
        let cmd = parsers::parse_Command("baud,9600").unwrap();
        assert_eq!(format!("{:?}", cmd), "Baud(Bps9600)");

        let cmd = parsers::parse_Command("baud,2400").unwrap();
        assert_eq!(format!("{:?}", cmd), "Baud(Bps2400)");
    }

    #[test]
    fn parsing_invalid_baud_commands_yields_err() {
        let cmd = parsers::parse_Command("baud,8600");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("baud,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_calibration_commands() {
        let cmd = parsers::parse_Command("caL,0.0").unwrap();
        assert_eq!(format!("{:?}", cmd), "Calibrate(0)");

        let cmd = parsers::parse_Command("caL,1.0").unwrap();
        assert_eq!(format!("{:?}", cmd), "Calibrate(1)");

        let cmd = parsers::parse_Command("caL,10.98857").unwrap();
        assert_eq!(format!("{:?}", cmd), "Calibrate(10.98857)");

        let cmd = parsers::parse_Command("cal,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "CalStatus");

        let cmd = parsers::parse_Command("CAL,CLEAR").unwrap();
        assert_eq!(format!("{:?}", cmd), "CalClear");
    }

    #[test]
    fn parsing_invalid_calibration_commands_yields_err() {
        let cmd = parsers::parse_Command("cal,!?");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("cal,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_datalogger_commands() {
        let cmd = parsers::parse_Command("D,0").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerOff");

        let cmd = parsers::parse_Command("D,320000").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerSet(320000)");

        let cmd = parsers::parse_Command("D,10").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerSet(10)");

        let cmd = parsers::parse_Command("D,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerStatus");
    }

    #[test]
    fn parsing_invalid_datalogger_commands_yields_err() {
        let cmd = parsers::parse_Command("d,!?");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("D,0.0");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("d,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_device_info_command() {
        let cmd = parsers::parse_Command("i").unwrap();
        assert_eq!(format!("{:?}", cmd), "Info");

        let cmd = parsers::parse_Command("I").unwrap();
        assert_eq!(format!("{:?}", cmd), "Info");
    }

    #[test]
    fn parsing_invalid_device_info_command_yields_err() {
        let cmd = parsers::parse_Command("i,?");
        println!("cmd: {:?}", cmd);
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("info");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("i,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_device_address_command() {
        let cmd = parsers::parse_Command("i2c,68").unwrap();
        assert_eq!(format!("{:?}", cmd), "I2c(68)");

        let cmd = parsers::parse_Command("I2C,128").unwrap();
        assert_eq!(format!("{:?}", cmd), "I2c(128)");
    }

    #[test]
    fn parsing_invalid_device_address_command_yields_err() {
        let cmd = parsers::parse_Command("i2c,?");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("i2c,10.5");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("i2c");
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("i2c,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_led_commands() {
        let cmd = parsers::parse_Command("L,0").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedOff");

        let cmd = parsers::parse_Command("L,1").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedOn");

        let cmd = parsers::parse_Command("L,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedStatus");
    }

    #[test]
    fn parsing_invalid_led_commands_yields_err() {
        let cmd = parsers::parse_Command("l,!?");
        println!("cmd: {:?}", cmd);
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("L,0.0");
        println!("cmd: {:?}", cmd);
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("L,2");
        println!("cmd: {:?}", cmd);
        assert!(cmd.is_err());

        let cmd = parsers::parse_Command("L,");
        assert!(cmd.is_err());
    }
}
