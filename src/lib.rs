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
    ProtocolLockStatus,
    Reading,
    ScaleCelsius,
    ScaleKelvin,
    ScaleFahrenheit,
    ScaleStatus,
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

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CommandResponse {
    code: Option<ResponseCode>,
    data: Option<[u8; 14]>
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CommandOptions {
    command: String,
    delay: Option<usize>,
    reponse: Option<CommandResponse>,
}

impl CommandOptions {
    /// Sets the ASCII string for the command to be sent
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions{
        self.command = command_str; self
    }
    fn set_delay(&mut self, delay: usize) -> &mut CommandOptions{
        self.delay = Some(delay); self
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
                .finish()
        },
        CalibrationClear => {
            CommandOptions::default()
                .set_command("Cal,clear\0".to_string())
                .set_delay(300)
                .finish()
        },
        CalibrationState => {
            CommandOptions::default()
                .set_command("Cal,?\0".to_string())
                .set_delay(300)
                .finish()

        },
        DataloggerPeriod(n) => {
            CommandOptions::default()
                .set_command(format!("D,{}\0", n))
                .set_delay(300)
                .finish()
        },
        DataloggerDisable => {
            CommandOptions::default()
                .set_command("D,0\0".to_string())
                .set_delay(300)
                .finish()
        },
        DataloggerInterval => {
            CommandOptions::default()
                .set_command("D,?\0".to_string())
                .set_delay(300)
                .finish()
        },
        DeviceAddress(addr) => {
            CommandOptions::default()
                .set_command(format!("I2C,{}\0", addr))
                .set_delay(300)
                .finish()
        },
        DeviceInformation => {
            CommandOptions::default()
                .set_command("I\0".to_string())
                .set_delay(300)
                .finish()
        },
        Export(ref calib) => {
            CommandOptions::default()
                .set_command(format!("Export,{}\0", calib))
                .set_delay(300)
                .finish()
        },
        ExportInfo => {
            CommandOptions::default()
                .set_command("Export,?\0".to_string())
                .set_delay(300)
                .finish()
        },
        Import(ref calib) => {
            CommandOptions::default()
                .set_command(format!("Import,{}\0", calib))
                .set_delay(300)
                .finish()
        },
        Factory => {
            CommandOptions::default()
                .set_command("Factory\0".to_string())
                .finish()
        },
        Find => {
            CommandOptions::default()
                .set_command("F\0".to_string())
                .set_delay(300)
                .finish()
        },
        LedOn => {
            CommandOptions::default()
                .set_command("L,1\0".to_string())
                .set_delay(300)
                .finish()
        },
        LedOff => {
            CommandOptions::default()
                .set_command("L,0\0".to_string())
                .set_delay(300)
                .finish()
        },
        LedState => {
            CommandOptions::default()
                .set_command("L,?\0".to_string())
                .set_delay(300)
                .finish()
        },
        MemoryClear => {
            CommandOptions::default()
                .set_command("M,clear\0".to_string())
                .set_delay(300)
                .finish()
        },
        MemoryRecall => {
            CommandOptions::default()
                .set_command("M\0".to_string())
                .set_delay(300)
                .finish()
        },
        MemoryRecallLastLocation => {
            CommandOptions::default()
                .set_command("M,?\0".to_string())
                .set_delay(300)
                .finish()
        },
        ProtocolLockEnable => {
            CommandOptions::default()
                .set_command("Plock,1\0".to_string())
                .set_delay(300)
                .finish()
        },
        ProtocolLockDisable => {
            CommandOptions::default()
                .set_command("Plock,0\0".to_string())
                .set_delay(300)
                .finish()
        },
        ProtocolLockStatus => {
            CommandOptions::default()
                .set_command("Plock,?\0".to_string())
                .set_delay(300)
                .finish()
        },
        Reading => {
            CommandOptions::default()
                .set_command("R\0".to_string())
                .set_delay(600)
                .finish()
        },
        ScaleCelsius => {
            CommandOptions::default()
                .set_command("S,c\0".to_string())
                .finish()
        },
        ScaleKelvin => {
            CommandOptions::default()
                .set_command("S,k\0".to_string())
                .set_delay(300)
                .finish()
        },
        ScaleFahrenheit => {
            CommandOptions::default()
                .set_command("S,f\0".to_string())
                .set_delay(300)
                .finish()
        },
        ScaleStatus => {
            CommandOptions::default()
                .set_command("S,?\0".to_string())
                .set_delay(300)
                .finish()
        },
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
        },
        Sleep => {
            CommandOptions::default()
                .set_command("Sleep\0".to_string())
                .finish()
        },
        Status => {
            CommandOptions::default()
                .set_command("Status\0".to_string())
                .set_delay(300)
                .finish()
        },
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

    fn temperature_command(cmd: TemperatureCommand) -> String {
        cmd.to_string()
    }

    #[test]
    fn temperature_command_uart_mode() {
        let cmd = temperature_command(SetUart(Bauds::Bps300));
        assert_eq!(cmd, "Baud,300\0");
        let cmd = temperature_command(SetUart(Bauds::Bps1200));
        assert_eq!(cmd, "Baud,1200\0");
        let cmd = temperature_command(SetUart(Bauds::Bps2400));
        assert_eq!(cmd, "Baud,2400\0");
        let cmd = temperature_command(SetUart(Bauds::Bps9600));
        assert_eq!(cmd, "Baud,9600\0");
        let cmd = temperature_command(SetUart(Bauds::Bps19200));
        assert_eq!(cmd, "Baud,19200\0");
        let cmd = temperature_command(SetUart(Bauds::Bps38400));
        assert_eq!(cmd, "Baud,38400\0");
        let cmd = temperature_command(SetUart(Bauds::Bps57600));
        assert_eq!(cmd, "Baud,57600\0");
        let cmd = temperature_command(SetUart(Bauds::Bps115200));
        assert_eq!(cmd, "Baud,115200\0");
    }

    #[test]
    fn temperature_command_calibration_temperature() {
        let cmd = temperature_command(CalibrationTemperature(35.2459));
        assert_eq!(cmd, "Cal,35.25\0");
    }

    #[test]
    fn temperature_command_calibration_clear() {
        let cmd = temperature_command(CalibrationClear);
        assert_eq!(cmd, "Cal,clear\0");
    }

    #[test]
    fn temperature_command_calibration_state() {
        let cmd = temperature_command(CalibrationState);
        assert_eq!(cmd, "Cal,?\0");
    }

    #[test]
    fn temperature_command_data_logger_period() {
        let cmd = temperature_command(DataloggerPeriod(10));
        assert_eq!(cmd, "D,10\0");
    }

    #[test]
    fn temperature_command_data_logger_disable() {
        let cmd = temperature_command(DataloggerDisable);
        assert_eq!(cmd, "D,0\0");
    }

    #[test]
    fn temperature_command_data_logger_interval() {
        let cmd = temperature_command(DataloggerInterval);
        assert_eq!(cmd, "D,?\0");
    }

    #[test]
    fn temperature_command_() {
        let cmd = temperature_command(DeviceAddress(88));
        assert_eq!(cmd, "I2C,88\0");
    }

    #[test]
    fn temperature_command_device_information() {
        let cmd = temperature_command(DeviceInformation);
        assert_eq!(cmd, "I\0");
    }

    #[test]
    fn temperature_command_export() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = temperature_command(Export(calibration_string));
        assert_eq!(cmd, "Export,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_export_info() {
        let cmd = temperature_command(ExportInfo);
        assert_eq!(cmd, "Export,?\0");
    }

    #[test]
    fn temperature_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = temperature_command(Import(calibration_string));
        assert_eq!(cmd, "Import,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_factory() {
        let cmd = temperature_command(Factory);
        assert_eq!(cmd, "Factory\0");
    }

    #[test]
    fn temperature_command_find() {
        let cmd = temperature_command(Find);
        assert_eq!(cmd, "F\0");
    }

    #[test]
    fn temperature_command_led_on() {
        let cmd = temperature_command(LedOn);
        assert_eq!(cmd, "L,1\0");
    }

    #[test]
    fn temperature_command_led_off() {
        let cmd = temperature_command(LedOff);
        assert_eq!(cmd, "L,0\0");
    }

    #[test]
    fn temperature_command_led_state() {
        let cmd = temperature_command(LedState);
        assert_eq!(cmd, "L,?\0");
    }

    #[test]
    fn temperature_command_memory_clear() {
        let cmd = temperature_command(MemoryClear);
        assert_eq!(cmd, "M,clear\0");
    }

    #[test]
    fn temperature_command_memory_recall() {
        let cmd = temperature_command(MemoryRecall);
        assert_eq!(cmd, "M\0");
    }

    #[test]
    fn temperature_command_memory_recall_location() {
        let cmd = temperature_command(MemoryRecallLastLocation);
        assert_eq!(cmd, "M,?\0");
    }

    #[test]
    fn temperature_command_plock_enable() {
        let cmd = temperature_command(ProtocolLockEnable);
        assert_eq!(cmd, "Plock,1\0");
    }

    #[test]
    fn temperature_command_plock_disable() {
        let cmd = temperature_command(ProtocolLockDisable);
        assert_eq!(cmd, "Plock,0\0");
    }

    #[test]
    fn temperature_command_plock_status() {
        let cmd = temperature_command(ProtocolLockStatus);
        assert_eq!(cmd, "Plock,?\0");
    }

    #[test]
    fn temperature_command_reading() {
        let cmd = temperature_command(Reading);
        assert_eq!(cmd, "R\0");
    }

    #[test]
    fn temperature_command_scale_celsius() {
        let cmd = temperature_command(ScaleCelsius);
        assert_eq!(cmd, "S,c\0");
    }

    #[test]
    fn temperature_command_scale_kelvin() {
        let cmd = temperature_command(ScaleKelvin);
        assert_eq!(cmd, "S,k\0");
    }

    #[test]
    fn temperature_command_scale_fahrenheit() {
        let cmd = temperature_command(ScaleFahrenheit);
        assert_eq!(cmd, "S,f\0");
    }

    #[test]
    fn temperature_command_scale_status() {
        let cmd = temperature_command(ScaleStatus);
        assert_eq!(cmd, "S,?\0");
    }

    #[test]
    fn temperature_command_sleep_mode() {
        let cmd = temperature_command(Sleep);
        assert_eq!(cmd, "Sleep\0");
    }

    #[test]
    fn temperature_command_device_status() {
        let cmd = temperature_command(Status);
        assert_eq!(cmd, "Status\0");
    }
}
