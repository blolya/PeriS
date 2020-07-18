pub mod gpioc;

use super::Clocked;
pub trait Gpio: Clocked {
    fn set_port_mode(&self, port: u32, mode: u32);
    fn get_port_mode(&self, port: u32) -> u32;
    fn set_port_config(&self, port: u32, config: u32);
    fn get_port_config(&self, port: u32) -> u32;
    fn set_port_output(&self, port: u32);
    fn reset_port_output(&self, port: u32);
    fn get_port_output(&self, port: u32) -> u32;
    fn get_port_input(&self, port: u32) -> u32;
}
pub struct Port<'a, T: Gpio> {
    port_num: PortNum,
    gpio: &'a T,
}
impl<'a, T: Gpio> Port<'a, T> {
    pub fn new(port_num: PortNum, port_type: PortType, gpio: &'a T) -> Port<'a, T> {
        let port = Port { port_num, gpio };
        port.set_port_type(port_type);
        port
    }
    pub fn set_high(&self) {
        let port_type = self.get_port_type();
        let port_num: u32 = (&self.port_num).into();
        if let PortType::Input(_) = port_type {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.set_port_output(port_num);
        }
    }
    pub fn set_low(&self) {
        let port_type = self.get_port_type();
        let port_num: u32 = (&self.port_num).into();
        if let PortType::Input(_) = port_type {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.reset_port_output(port_num);
        }
    }
    pub fn set_port_type(&self, port_type: PortType) {
        let (bin_config, bin_speed) = port_type.into();
        let port_num: u32 = (&self.port_num).into();
        self.gpio.set_port_config(port_num, bin_config);
        self.gpio.set_port_mode(port_num, bin_speed);
    }
    pub fn get_port_type(&self) -> PortType {
        let port_num: u32 = (&self.port_num).into();
        let bin_config = self.gpio.get_port_config(port_num);
        let bin_mode = self.gpio.get_port_mode(port_num);
        let mode: PortType = (bin_config, bin_mode).into();
        mode
    }
}

pub enum OutputConfig {
    GeneralPurposePushPull(PortMode),
    GeneralPurposeOpenDrain(PortMode),
    AlternativeFunctionPushPull(PortMode),
    AlternativeFunctionOpenDrain(PortMode),
}
impl From<OutputConfig> for (u32, u32) {
    fn from(config: OutputConfig) -> (u32, u32) {
        match config {
            OutputConfig::GeneralPurposePushPull(speed) => (0, speed as u32),
            OutputConfig::GeneralPurposeOpenDrain(speed) => (1, speed as u32),
            OutputConfig::AlternativeFunctionPushPull(speed) => (2, speed as u32),
            OutputConfig::AlternativeFunctionOpenDrain(speed) => (3, speed as u32),
        }
    }
}
impl From<(u32, u32)> for OutputConfig {
    fn from( (bin_config, bin_mode): (u32, u32) ) -> OutputConfig {
        let speed: PortMode = bin_mode.into();
        match bin_config {
            0 => OutputConfig::GeneralPurposePushPull(speed),
            1 => OutputConfig::GeneralPurposeOpenDrain(speed),
            2 => OutputConfig::AlternativeFunctionPushPull(speed),
            3 => OutputConfig::AlternativeFunctionOpenDrain(speed),
            _ => panic!(""),
        }
    }
}
pub enum InputConfig {
    Analog,
    Floating,
    PullDown,
    PullUp,
}
impl From<InputConfig> for (u32, u32) {
    fn from(config: InputConfig) -> (u32, u32) {
        match config {
            InputConfig::Analog => (0, 0),
            InputConfig::Floating => (1, 0),
            InputConfig::PullDown => (2, 0),
            InputConfig::PullUp => (2, 0),   
        }
    }
}
impl From <(u32, u32)> for InputConfig {
    fn from( (bin_config, _): (u32, u32) ) -> InputConfig {
        match bin_config {
            0 => InputConfig::Analog,
            1 => InputConfig::Floating,
            2 => InputConfig::PullDown,
            _ => panic!("")
        }
    }
}
pub enum PortType {
    Input(InputConfig),
    Output(OutputConfig),
}
impl From<PortType> for (u32, u32) {
    fn from(port_type: PortType) -> (u32, u32) {
        match port_type {
            PortType::Input(config) => {
                config.into()
            },
            PortType::Output(config) => {
                config.into()
            }
        }
    }
}
impl From<(u32, u32)> for PortType {
    fn from( (bin_config, bin_mode): (u32, u32) ) -> PortType {
        if bin_mode == 0 {
            let config: InputConfig = (bin_config, bin_mode).into();
            PortType::Input(config)
        } else {
            let config: OutputConfig = (bin_config, bin_mode).into();
            PortType::Output(config)
        }
    }
}

pub enum PortNum {
    P13,
}
impl From<PortNum> for u32 {
    fn from(port_num: PortNum) -> u32 {
        match port_num {
            PortNum::P13 => 13,
            _ => panic!(""),
        }
    }
}
impl From<&PortNum> for u32 {
    fn from(port_num: &PortNum) -> u32 {
        match port_num {
            PortNum::P13 => 13,
            _ => panic!(""),
        }
    }
}
impl From<u32> for PortNum {
    fn from(port_num: u32) -> PortNum {
        match port_num {
            13 => PortNum::P13,
            _ => panic!("Hey"),
        }
    }
}
pub enum PortMode {
    Reserved = 0,
    S2MHz = 2,
    S10MHz = 1,
    S50MHz = 3,
}
impl From<u32> for PortMode {
    fn from(bin_speed: u32) -> PortMode {
        match bin_speed {
            0 => PortMode::Reserved,
            1 => PortMode::S10MHz,
            2 => PortMode::S2MHz,
            3 => PortMode::S50MHz,
            _ => panic!(""),
        }
    }
}