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
        let cmd = command::ApiCommand::parse("baud,9600").unwrap();
        assert_eq!(format!("{:?}", cmd), "Baud(Bps9600)");

        let cmd = command::ApiCommand::parse("baud,2400").unwrap();
        assert_eq!(format!("{:?}", cmd), "Baud(Bps2400)");
    }

    #[test]
    fn parsing_invalid_baud_commands_yields_err() {
        let cmd = command::ApiCommand::parse("baud,8600");
        assert!(cmd.is_err());

        let cmd = command::ApiCommand::parse("baud,");
        assert!(cmd.is_err());
    }

    #[test]
    fn parses_calibration_commands() {
        let cmd = command::ApiCommand::parse("caL,1.0").unwrap();
        assert_eq!(format!("{:?}", cmd), "Calibrate(1)");

        let cmd = parsers::parse_Command("caL,10.98857").unwrap();
        assert_eq!(format!("{:?}", cmd), "Calibrate(10.98857)");

        let cmd = parsers::parse_Command("cal,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "CalStatus");

        let cmd = parsers::parse_Command("CAL,CLEAR").unwrap();
        assert_eq!(format!("{:?}", cmd), "CalClear");
    }

    #[test]
    fn parses_datalogger_commands() {
        let cmd = command::ApiCommand::parse("D,0").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerOff");

        let cmd = parsers::parse_Command("D,1").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerSet(1)");

        let cmd = parsers::parse_Command("D,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "DataLoggerStatus");
    }

    #[test]
    fn parses_led_commands() {
        let cmd = command::ApiCommand::parse("L,0").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedOff");

        let cmd = parsers::parse_Command("L,1").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedOn");

        let cmd = parsers::parse_Command("L,?").unwrap();
        assert_eq!(format!("{:?}", cmd), "LedStatus");
    }
}
