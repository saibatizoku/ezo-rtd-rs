use errors::*;
use ezo_common::{ResponseCode, parse_data_ascii_bytes, response_code, write_to_ezo,
                 read_raw_buffer};
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
    pub data: Option<Vec<u8>>,
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
    fn delay(&self) -> Result<()>;
    fn finish(&self) -> Self;
    fn parse_response(&self) -> Result<String>;
    fn read_response(&mut self, dev: &mut LinuxI2CDevice) -> Result<()>;
    fn run(&mut self, dev: &mut LinuxI2CDevice) -> Result<()>;
    fn set_command(&mut self, command_str: String) -> &mut Self;
    fn set_delay(&mut self, delay: u64) -> &mut Self;
    fn set_response(&mut self, response: CommandResponse) -> &mut Self;
    fn write(&mut self, dev: &mut LinuxI2CDevice) -> Result<()>;
}

impl CommandBuilder for CommandOptions {
    fn delay(&self) -> Result<()> {
        if let Some(delay) = self.delay {
            thread::sleep(Duration::from_millis(delay));
        };
        Ok(())
    }
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
    fn run(&mut self, dev: &mut LinuxI2CDevice) -> Result<()> {
        self.write(dev)?;
        self.delay()?;
        self.read_response(dev)?;
        Ok(())
    }
    fn write(&mut self, dev: &mut LinuxI2CDevice) -> Result<()> {
        write_to_ezo(dev, self.command.as_bytes()).chain_err(|| "Error writing to EZO device.")
    }
    fn read_response(&mut self, dev: &mut LinuxI2CDevice) -> Result<()> {
        if let Some(_) = self.response {
            let data = read_raw_buffer(dev, MAX_RESPONSE_LENGTH)?;
            self.data = Some(data);
        };
        Ok(())
    }
    fn parse_response(&self) -> Result<String> {
        match self.data {
            Some(ref data) => {
                match response_code(data[0]) {
                    ResponseCode::Success => {
                        String::from_utf8(parse_data_ascii_bytes(&data[1..]))
                            .chain_err(|| "Data is not parsable")
                    }
                    _ => Ok(String::new()),
                }
            }
            _ => Ok(String::new()),
        }
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
