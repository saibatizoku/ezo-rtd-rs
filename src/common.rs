use errors::*;
use ezo_common::{ResponseCode, parse_data_ascii_bytes, response_code};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

/// Maximum ascii-character response size + 2
pub const MAX_RESPONSE_LENGTH: usize = 16;

/// Command-related parameters used to build I2C write/read interactions.
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct CommandOptions {
    pub command: String,
    pub delay: Option<u64>,
    pub response: Option<CommandResponse>,
}

/// Allowed responses from I2C read interactions.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum CommandResponse {
    Ack,
    CalibrationState,
    DataloggerInterval,
    DeviceInformation,
    ExportInfo,
    Export,
    LedState,
    MemoryRecall,
    MemoryRecallLastLocation,
    ProtocolLockState,
    Reading,
    ScaleState,
    Status,
}

/// Builds commands.
pub trait CommandBuilder {
    fn finish(&self) -> Self;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<String>;
    fn set_command(&mut self, command_str: String) -> &mut Self;
    fn set_delay(&mut self, delay: u64) -> &mut Self;
    fn set_response(&mut self, response: CommandResponse) -> &mut Self;
}

impl CommandBuilder for CommandOptions {
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<String> {
        if let Err(_) = dev.write(self.command.as_bytes()) {
            thread::sleep(Duration::from_millis(300));
            dev.write(self.command.as_bytes())
                .chain_err(|| "Command could not be sent")?;
        };
        if let Some(delay) = self.delay {
            thread::sleep(Duration::from_millis(delay));
        }
        if let Some(_) = self.response {
            let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
            if let Err(_) = dev.read(&mut data_buffer) {
                thread::sleep(Duration::from_millis(300));
                dev.read(&mut data_buffer)
                    .chain_err(|| "Error reading from device")?;
            };
            match response_code(data_buffer[0]) {
                ResponseCode::NoDataExpected => println!("No data expected."),
                ResponseCode::Pending => println!("Pending"),
                ResponseCode::DeviceError => println!("Error"),
                ResponseCode::Success => {
                    return Ok(String::from_utf8(parse_data_ascii_bytes(&data_buffer[1..]))
                                  .chain_err(|| "Data is not parsable")?)
                }
                ResponseCode::UnknownError => println!("NO RESPONSE"),
            };
        }
        Ok(String::new())
    }

    /// Sets the ASCII string for the command to be sent
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions {
        self.command = command_str;
        self
    }
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions {
        self.delay = Some(delay);
        self
    }
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions {
        self.response = Some(response);
        self
    }
}

/// Useful for properly building I2C parameters from a command.
pub trait I2cCommand {
    fn build(&self) -> CommandOptions;
}
