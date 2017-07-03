use errors::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

/// Maximum ascii-character response size + 2
pub const MAX_RESPONSE_LENGTH: usize = 16;

/// Allowable baudrates used when changing the chip to UART mode.
#[derive(Debug)]
pub enum BpsRate {
    Bps300 = 300,
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

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
    fn finish(&self) -> CommandOptions;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<String>;
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions;
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions;
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions;
}

impl CommandBuilder for CommandOptions {
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<String> {
        let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
        if let Err(_) = dev.write(self.command.as_bytes()) {
            thread::sleep(Duration::from_millis(300));
            dev.write(self.command.as_bytes())
                .chain_err(|| "Command could not be sent")?;
        };
        if let Some(delay) = self.delay {
            thread::sleep(Duration::from_millis(delay));
        }
        if let Some(_) = self.response {
            if let Err(_) = dev.read(&mut data_buffer) {
                thread::sleep(Duration::from_millis(300));
                dev.read(&mut data_buffer)
                    .chain_err(|| "Error reading from device")?;
            };
            match data_buffer[0] {
                255 => println!("No data expected."),
                254 => println!("Pending"),
                2 => println!("Error"),
                1 => {
                    let data: String = match data_buffer.into_iter().position(|&x| x == 0) {
                        Some(eol) => {
                            data_buffer[1..eol]
                                .into_iter()
                                .map(|c| (*c & !0x80) as char)
                                .collect()
                        }
                        _ => {
                            String::from_utf8(Vec::from(&data_buffer[1..]))
                                .chain_err(|| "Data is not readable")?
                        }
                    };
                    return Ok(data);
                }
                _ => println!("NO RESPONSE"),
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
