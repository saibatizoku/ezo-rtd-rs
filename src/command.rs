//! I2C commands for the RTD EZO Chip.
//! 
use {MAX_DATA, LinuxI2CDevice};
use errors::*;

pub trait Command {
    type Response;

    fn get_command_string (&self) -> String;
    fn get_delay (&self) -> u64;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Self::Response>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_temperature_scale_command() {
        unimplemented!();
    }

    #[test]
    fn builds_sleep_command() {
        unimplemented!();
    }
}
