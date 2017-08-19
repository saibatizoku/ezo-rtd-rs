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

/// Errors handled.
pub mod errors;

/// Issuable commands for the EZO RTD Chip.
pub mod command;

/// Parseable responses from the EZO RTD Chip.
pub mod response;
