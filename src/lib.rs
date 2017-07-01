//! I2C Commands for RTD EZO Chip, taken from their Datasheet.
//! This chip is used for temperature measurement. It features
//! calibration, sleep mode, scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

pub mod errors;
use errors::*;

pub const MAX_RESPONSE_LENGTH: usize = 16;

pub trait I2cCommand {
    fn build(&self) -> CommandOptions;
}

#[derive(Debug)]
pub enum Bauds {
    Bps300 = 300,
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

#[derive(Debug)]
pub enum TemperatureCommand {
    CalibrationTemperature(f64),
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod(u16),
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress(u16),
    DeviceInformation,
    Export(String),
    ExportInfo,
    Import(String),
    Factory,
    Find,
    LedOn,
    LedOff,
    LedState,
    MemoryClear,
    MemoryRecall,
    MemoryRecallLastLocation,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockState,
    Reading,
    ScaleCelsius,
    ScaleKelvin,
    ScaleFahrenheit,
    ScaleState,
    SetUart(Bauds),
    Sleep,
    Status,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ResponseCode {
    NoDataExpected = 0xFF,
    Pending = 0xFE,
    Error = 0x02,
    Success = 0x01,
}

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

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct CommandOptions {
    pub command: String,
    pub delay: Option<u64>,
    pub response: Option<CommandResponse>,
}

pub trait CommandBuilder {
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions;
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions;
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions;
    fn finish(&self) -> CommandOptions;
}

impl CommandBuilder for CommandOptions {
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
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
}

impl I2cCommand for TemperatureCommand {
    fn build(&self) -> CommandOptions {
        use self::TemperatureCommand::*;
        let mut opts = CommandOptions::default();
        match *self {
            CalibrationTemperature(temp) => {
                opts.set_command(format!("Cal,{:.*}\0", 2, temp))
                    .set_delay(1000)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationClear => {
                opts.set_command("Cal,clear\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationState => {
                opts.set_command("Cal,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::CalibrationState)
                    .finish()
            }
            DataloggerPeriod(n) => {
                opts.set_command(format!("D,{}\0", n))
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            DataloggerDisable => {
                opts.set_command("D,0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            DataloggerInterval => {
                opts.set_command("D,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::DataloggerInterval)
                    .finish()
            }
            DeviceAddress(addr) => {
                opts.set_command(format!("I2C,{}\0", addr))
                    .set_delay(300)
                    .finish()
            }
            DeviceInformation => {
                opts.set_command("I\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::DeviceInformation)
                    .finish()
            }
            Export(ref calib) => {
                opts.set_command(format!("Export,{}\0", calib))
                    .set_delay(300)
                    .set_response(CommandResponse::Export)
                    .finish()
            }
            ExportInfo => {
                opts.set_command("Export,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ExportInfo)
                    .finish()
            }
            Import(ref calib) => {
                opts.set_command(format!("Import,{}\0", calib))
                    .set_delay(300)
                    .finish()
            }
            Factory => opts.set_command("Factory\0".to_string()).finish(),
                    Find => {
                        opts.set_command("F\0".to_string())
                            .set_delay(300)
                            .set_response(CommandResponse::Ack)
                            .finish()
                    }
            LedOn => {
                opts.set_command("L,1\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            LedOff => {
                opts.set_command("L,0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            LedState => {
                opts.set_command("L,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::LedState)
                    .finish()
            }
            MemoryClear => {
                opts.set_command("M,clear\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            MemoryRecall => {
                opts.set_command("M\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::MemoryRecall)
                    .finish()
            }
            MemoryRecallLastLocation => {
                opts.set_command("M,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::MemoryRecallLastLocation)
                    .finish()
            }
            ProtocolLockEnable => {
                opts.set_command("Plock,1\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProtocolLockDisable => {
                opts.set_command("Plock,0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProtocolLockState => {
                opts.set_command("Plock,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ProtocolLockState)
                    .finish()
            }
            Reading => {
                opts.set_command("R\0".to_string())
                    .set_delay(600)
                    .set_response(CommandResponse::Reading)
                    .finish()
            }
            ScaleCelsius => {
                opts.set_command("S,c\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ScaleKelvin => {
                opts.set_command("S,k\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ScaleFahrenheit => {
                opts.set_command("S,f\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ScaleState => {
                opts.set_command("S,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ScaleState)
                    .finish()
            }
            SetUart(ref baud) => {
                let rate = match *baud {
                    Bauds::Bps300 => Bauds::Bps300 as u32,
                    Bauds::Bps1200 => Bauds::Bps1200 as u32,
                    Bauds::Bps2400 => Bauds::Bps2400 as u32,
                    Bauds::Bps9600 => Bauds::Bps9600 as u32,
                    Bauds::Bps19200 => Bauds::Bps19200 as u32,
                    Bauds::Bps38400 => Bauds::Bps38400 as u32,
                    Bauds::Bps57600 => Bauds::Bps57600 as u32,
                    Bauds::Bps115200 => Bauds::Bps115200 as u32,
                };
                opts.set_command(format!("Baud,{}\0", rate)).finish()
            }
            Sleep => opts.set_command("Sleep\0".to_string()).finish(),
            Status => {
                opts.set_command("Status\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Status)
                    .finish()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::TemperatureCommand::*;

    #[test]
    fn temperature_command_uart_300() {
        let cmd = SetUart(Bauds::Bps300).build();
        assert_eq!(cmd.command, "Baud,300\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_1200() {
        let cmd = SetUart(Bauds::Bps1200).build();
        assert_eq!(cmd.command, "Baud,1200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_2400() {
        let cmd = SetUart(Bauds::Bps2400).build();
        assert_eq!(cmd.command, "Baud,2400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_9600() {
        let cmd = SetUart(Bauds::Bps9600).build();
        assert_eq!(cmd.command, "Baud,9600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_19200() {
        let cmd = SetUart(Bauds::Bps19200).build();
        assert_eq!(cmd.command, "Baud,19200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_38400() {
        let cmd = SetUart(Bauds::Bps38400).build();
        assert_eq!(cmd.command, "Baud,38400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_57600() {
        let cmd = SetUart(Bauds::Bps57600).build();
        assert_eq!(cmd.command, "Baud,57600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_uart_115200() {
        let cmd = SetUart(Bauds::Bps115200).build();
        assert_eq!(cmd.command, "Baud,115200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_calibration_temperature() {
        let cmd = CalibrationTemperature(35.2459).build();
        assert_eq!(cmd.command, "Cal,35.25\0");
        assert_eq!(cmd.delay, Some(1000));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_calibration_clear() {
        let cmd = CalibrationClear.build();
        assert_eq!(cmd.command, "Cal,clear\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_calibration_state() {
        let cmd = CalibrationState.build();
        assert_eq!(cmd.command, "Cal,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::CalibrationState));
    }

    #[test]
    fn temperature_command_data_logger_period() {
        let cmd = DataloggerPeriod(10).build();
        assert_eq!(cmd.command, "D,10\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_data_logger_disable() {
        let cmd = DataloggerDisable.build();
        assert_eq!(cmd.command, "D,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_data_logger_interval() {
        let cmd = DataloggerInterval.build();
        assert_eq!(cmd.command, "D,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::DataloggerInterval));
    }

    #[test]
    fn temperature_command_change_device_address() {
        let cmd = DeviceAddress(88).build();
        assert_eq!(cmd.command, "I2C,88\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_device_information() {
        let cmd = DeviceInformation.build();
        assert_eq!(cmd.command, "I\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::DeviceInformation));
    }

    #[test]
    fn temperature_command_export() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Export(calibration_string).build();
        assert_eq!(cmd.command, "Export,ABCDEFGHIJKLMNO\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Export));
    }

    #[test]
    fn temperature_command_export_info() {
        let cmd = ExportInfo.build();
        assert_eq!(cmd.command, "Export,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ExportInfo));
    }

    #[test]
    fn temperature_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string).build();
        assert_eq!(cmd.command, "Import,ABCDEFGHIJKLMNO\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_factory() {
        let cmd = Factory.build();
        assert_eq!(cmd.command, "Factory\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_find() {
        let cmd = Find.build();
        assert_eq!(cmd.command, "F\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_led_on() {
        let cmd = LedOn.build();
        assert_eq!(cmd.command, "L,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_led_off() {
        let cmd = LedOff.build();
        assert_eq!(cmd.command, "L,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_led_state() {
        let cmd = LedState.build();
        assert_eq!(cmd.command, "L,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::LedState));
    }

    #[test]
    fn temperature_command_memory_clear() {
        let cmd = MemoryClear.build();
        assert_eq!(cmd.command, "M,clear\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_memory_recall() {
        let cmd = MemoryRecall.build();
        assert_eq!(cmd.command, "M\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::MemoryRecall));
    }

    #[test]
    fn temperature_command_memory_recall_location() {
        let cmd = MemoryRecallLastLocation.build();
        assert_eq!(cmd.command, "M,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response,
                   Some(CommandResponse::MemoryRecallLastLocation));
    }

    #[test]
    fn temperature_command_plock_enable() {
        let cmd = ProtocolLockEnable.build();
        assert_eq!(cmd.command, "Plock,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_plock_disable() {
        let cmd = ProtocolLockDisable.build();
        assert_eq!(cmd.command, "Plock,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_plock_status() {
        let cmd = ProtocolLockState.build();
        assert_eq!(cmd.command, "Plock,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ProtocolLockState));
    }

    #[test]
    fn temperature_command_reading() {
        let cmd = Reading.build();
        assert_eq!(cmd.command, "R\0");
        assert_eq!(cmd.delay, Some(600));
        assert_eq!(cmd.response, Some(CommandResponse::Reading));
    }

    #[test]
    fn temperature_command_scale_celsius() {
        let cmd = ScaleCelsius.build();
        assert_eq!(cmd.command, "S,c\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_scale_kelvin() {
        let cmd = ScaleKelvin.build();
        assert_eq!(cmd.command, "S,k\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_scale_fahrenheit() {
        let cmd = ScaleFahrenheit.build();
        assert_eq!(cmd.command, "S,f\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn temperature_command_scale_status() {
        let cmd = ScaleState.build();
        assert_eq!(cmd.command, "S,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ScaleState));
    }

    #[test]
    fn temperature_command_sleep_mode() {
        let cmd = Sleep.build();
        assert_eq!(cmd.command, "Sleep\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_device_status() {
        let cmd = Status.build();
        assert_eq!(cmd.command, "Status\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Status));
    }
}
