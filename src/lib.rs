//! I2C Commands for EZO RTD Chip, taken from their Datasheet.
//! This chip is used for temperature measurement. It features
//! calibration, sleep mode, scale, etc.
extern crate failure;
#[macro_use]
extern crate ezo_common;
extern crate i2cdev;

/// Issuable commands for the EZO RTD Chip.
pub mod command;

/// Parseable responses from the EZO RTD Chip.
pub mod response;

// Re-export errors from ezo_common crate.
pub use ezo_common::errors::{ErrorKind, EzoError};
