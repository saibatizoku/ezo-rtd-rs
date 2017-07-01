//! I2C Commands for RTD EZO Chip, taken from their Datasheet.
//! This chip is used for temperature measurement. It features
//! calibration, sleep mode, scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

pub mod errors;
use errors::*;

pub trait I2cCommand {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

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
enum CommandResponse {
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
struct CommandOptions {
    command: String,
    delay: Option<usize>,
    response: Option<CommandResponse>,
}

impl CommandOptions {
    /// Sets the ASCII string for the command to be sent
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions {
        self.command = command_str;
        self
    }
    fn set_delay(&mut self, delay: usize) -> &mut CommandOptions {
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

fn build_command(cmd: &TemperatureCommand) -> CommandOptions {
    use self::TemperatureCommand::*;
    match *cmd {
        CalibrationTemperature(temp) => {
            CommandOptions::default()
                .set_command(format!("Cal,{:.*}\0", 2, temp))
                .set_delay(1000)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        CalibrationClear => {
            CommandOptions::default()
                .set_command("Cal,clear\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        CalibrationState => {
            CommandOptions::default()
                .set_command("Cal,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::CalibrationState)
                .finish()
        }
        DataloggerPeriod(n) => {
            CommandOptions::default()
                .set_command(format!("D,{}\0", n))
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        DataloggerDisable => {
            CommandOptions::default()
                .set_command("D,0\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        DataloggerInterval => {
            CommandOptions::default()
                .set_command("D,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::DataloggerInterval)
                .finish()
        }
        DeviceAddress(addr) => {
            CommandOptions::default()
                .set_command(format!("I2C,{}\0", addr))
                .set_delay(300)
                .finish()
        }
        DeviceInformation => {
            CommandOptions::default()
                .set_command("I\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::DeviceInformation)
                .finish()
        }
        Export(ref calib) => {
            CommandOptions::default()
                .set_command(format!("Export,{}\0", calib))
                .set_delay(300)
                .set_response(CommandResponse::Export)
                .finish()
        }
        ExportInfo => {
            CommandOptions::default()
                .set_command("Export,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::ExportInfo)
                .finish()
        }
        Import(ref calib) => {
            CommandOptions::default()
                .set_command(format!("Import,{}\0", calib))
                .set_delay(300)
                .finish()
        }
        Factory => {
            CommandOptions::default()
                .set_command("Factory\0".to_string())
                .finish()
        }
        Find => {
            CommandOptions::default()
                .set_command("F\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        LedOn => {
            CommandOptions::default()
                .set_command("L,1\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        LedOff => {
            CommandOptions::default()
                .set_command("L,0\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        LedState => {
            CommandOptions::default()
                .set_command("L,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::LedState)
                .finish()
        }
        MemoryClear => {
            CommandOptions::default()
                .set_command("M,clear\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        MemoryRecall => {
            CommandOptions::default()
                .set_command("M\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::MemoryRecall)
                .finish()
        }
        MemoryRecallLastLocation => {
            CommandOptions::default()
                .set_command("M,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::MemoryRecallLastLocation)
                .finish()
        }
        ProtocolLockEnable => {
            CommandOptions::default()
                .set_command("Plock,1\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        ProtocolLockDisable => {
            CommandOptions::default()
                .set_command("Plock,0\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        ProtocolLockState => {
            CommandOptions::default()
                .set_command("Plock,?\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::ProtocolLockState)
                .finish()
        }
        Reading => {
            CommandOptions::default()
                .set_command("R\0".to_string())
                .set_delay(600)
                .set_response(CommandResponse::Reading)
                .finish()
        }
        ScaleCelsius => {
            CommandOptions::default()
                .set_command("S,c\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        ScaleKelvin => {
            CommandOptions::default()
                .set_command("S,k\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        ScaleFahrenheit => {
            CommandOptions::default()
                .set_command("S,f\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Ack)
                .finish()
        }
        ScaleState => {
            CommandOptions::default()
                .set_command("S,?\0".to_string())
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
            CommandOptions::default()
                .set_command(format!("Baud,{}\0", rate))
                .finish()
        }
        Sleep => {
            CommandOptions::default()
                .set_command("Sleep\0".to_string())
                .finish()
        }
        Status => {
            CommandOptions::default()
                .set_command("Status\0".to_string())
                .set_delay(300)
                .set_response(CommandResponse::Status)
                .finish()
        }
    }
}

fn command_string(cmd: &TemperatureCommand) -> String {
    build_command(cmd).command
}

impl I2cCommand for TemperatureCommand {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    fn to_string(&self) -> String {
        command_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::TemperatureCommand::*;

    #[test]
    fn temperature_command_uart_mode() {
        let cmd = build_command(&SetUart(Bauds::Bps300));
        assert_eq!(cmd.command, "Baud,300\0");
        let cmd = build_command(&SetUart(Bauds::Bps1200));
        assert_eq!(cmd.command, "Baud,1200\0");
        let cmd = build_command(&SetUart(Bauds::Bps2400));
        assert_eq!(cmd.command, "Baud,2400\0");
        let cmd = build_command(&SetUart(Bauds::Bps9600));
        assert_eq!(cmd.command, "Baud,9600\0");
        let cmd = build_command(&SetUart(Bauds::Bps19200));
        assert_eq!(cmd.command, "Baud,19200\0");
        let cmd = build_command(&SetUart(Bauds::Bps38400));
        assert_eq!(cmd.command, "Baud,38400\0");
        let cmd = build_command(&SetUart(Bauds::Bps57600));
        assert_eq!(cmd.command, "Baud,57600\0");
        let cmd = build_command(&SetUart(Bauds::Bps115200));
        assert_eq!(cmd.command, "Baud,115200\0");
    }

    #[test]
    fn temperature_command_calibration_temperature() {
        let cmd = build_command(&CalibrationTemperature(35.2459));
        assert_eq!(cmd.command, "Cal,35.25\0");
    }

    #[test]
    fn temperature_command_calibration_clear() {
        let cmd = build_command(&CalibrationClear);
        assert_eq!(cmd.command, "Cal,clear\0");
    }

    #[test]
    fn temperature_command_calibration_state() {
        let cmd = build_command(&CalibrationState);
        assert_eq!(cmd.command, "Cal,?\0");
    }

    #[test]
    fn temperature_command_data_logger_period() {
        let cmd = build_command(&DataloggerPeriod(10));
        assert_eq!(cmd.command, "D,10\0");
    }

    #[test]
    fn temperature_command_data_logger_disable() {
        let cmd = build_command(&DataloggerDisable);
        assert_eq!(cmd.command, "D,0\0");
    }

    #[test]
    fn temperature_command_data_logger_interval() {
        let cmd = build_command(&DataloggerInterval);
        assert_eq!(cmd.command, "D,?\0");
    }

    #[test]
    fn temperature_command_() {
        let cmd = build_command(&DeviceAddress(88));
        assert_eq!(cmd.command, "I2C,88\0");
    }

    #[test]
    fn temperature_command_device_information() {
        let cmd = build_command(&DeviceInformation);
        assert_eq!(cmd.command, "I\0");
    }

    #[test]
    fn temperature_command_export() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = build_command(&Export(calibration_string));
        assert_eq!(cmd.command, "Export,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_export_info() {
        let cmd = build_command(&ExportInfo);
        assert_eq!(cmd.command, "Export,?\0");
    }

    #[test]
    fn temperature_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = build_command(&Import(calibration_string));
        assert_eq!(cmd.command, "Import,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_factory() {
        let cmd = build_command(&Factory);
        assert_eq!(cmd.command, "Factory\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_find() {
        let cmd = build_command(&Find);
        assert_eq!(cmd.command, "F\0");
    }

    #[test]
    fn temperature_command_led_on() {
        let cmd = build_command(&LedOn);
        assert_eq!(cmd.command, "L,1\0");
    }

    #[test]
    fn temperature_command_led_off() {
        let cmd = build_command(&LedOff);
        assert_eq!(cmd.command, "L,0\0");
    }

    #[test]
    fn temperature_command_led_state() {
        let cmd = build_command(&LedState);
        assert_eq!(cmd.command, "L,?\0");
    }

    #[test]
    fn temperature_command_memory_clear() {
        let cmd = build_command(&MemoryClear);
        assert_eq!(cmd.command, "M,clear\0");
    }

    #[test]
    fn temperature_command_memory_recall() {
        let cmd = build_command(&MemoryRecall);
        assert_eq!(cmd.command, "M\0");
    }

    #[test]
    fn temperature_command_memory_recall_location() {
        let cmd = build_command(&MemoryRecallLastLocation);
        assert_eq!(cmd.command, "M,?\0");
    }

    #[test]
    fn temperature_command_plock_enable() {
        let cmd = build_command(&ProtocolLockEnable);
        assert_eq!(cmd.command, "Plock,1\0");
    }

    #[test]
    fn temperature_command_plock_disable() {
        let cmd = build_command(&ProtocolLockDisable);
        assert_eq!(cmd.command, "Plock,0\0");
    }

    #[test]
    fn temperature_command_plock_status() {
        let cmd = build_command(&ProtocolLockState);
        assert_eq!(cmd.command, "Plock,?\0");
    }

    #[test]
    fn temperature_command_reading() {
        let cmd = build_command(&Reading);
        assert_eq!(cmd.command, "R\0");
    }

    #[test]
    fn temperature_command_scale_celsius() {
        let cmd = build_command(&ScaleCelsius);
        assert_eq!(cmd.command, "S,c\0");
    }

    #[test]
    fn temperature_command_scale_kelvin() {
        let cmd = build_command(&ScaleKelvin);
        assert_eq!(cmd.command, "S,k\0");
    }

    #[test]
    fn temperature_command_scale_fahrenheit() {
        let cmd = build_command(&ScaleFahrenheit);
        assert_eq!(cmd.command, "S,f\0");
    }

    #[test]
    fn temperature_command_scale_status() {
        let cmd = build_command(&ScaleState);
        assert_eq!(cmd.command, "S,?\0");
    }

    #[test]
    fn temperature_command_sleep_mode() {
        let cmd = build_command(&Sleep);
        assert_eq!(cmd.command, "Sleep\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn temperature_command_device_status() {
        let cmd = build_command(&Status);
        assert_eq!(cmd.command, "Status\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Status));
    }
}
