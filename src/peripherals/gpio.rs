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
    pub fn new(port_num: PortNum, gpio: &'a T, mode: Mode) -> Port<'a, T> {
        let port = Port { port_num, gpio };
        port.set_mode(mode);
        port
    }
    pub fn set_high(&self) {
        let mode = self.get_mode();
        let port_num: u32 = (&self.port_num).into();
        if let Mode::Input(_) = mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.set_port_output(port_num);
        }
    }
    pub fn set_low(&self) {
        let mode = self.get_mode();
        let port_num: u32 = (&self.port_num).into();
        if let Mode::Input(_) = mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.reset_port_output(port_num);
        }
    }
    pub fn set_mode(&self, mode: Mode) {
        let (bin_config, bin_speed) = mode.into();
        let port_num: u32 = (&self.port_num).into();
        self.gpio.set_port_config(port_num, bin_config);
        self.gpio.set_port_mode(port_num, bin_speed);
    }
    pub fn get_mode(&self) -> Mode {
        let port_num: u32 = (&self.port_num).into();
        let bin_config = self.gpio.get_port_config(port_num);
        let bin_mode = self.gpio.get_port_mode(port_num);
        let mode: Mode = (bin_config, bin_mode).into();
        mode
    }
}

pub enum OutputConfig {
    GeneralPurposePushPull(PortMaxSpeed),
    GeneralPurposeOpenDrain(PortMaxSpeed),
    AlternativeFunctionPushPull(PortMaxSpeed),
    AlternativeFunctionOpenDrain(PortMaxSpeed),
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
    fn from( (bin_config, bin_speed): (u32, u32) ) -> OutputConfig {
        let speed: PortMaxSpeed = bin_speed.into();
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
pub enum Mode {
    Input(InputConfig),
    Output(OutputConfig),
}
impl From<Mode> for (u32, u32) {
    fn from(mode: Mode) -> (u32, u32) {
        match mode {
            Mode::Input(config) => {
                config.into()
            },
            Mode::Output(config) => {
                config.into()
            }
        }
    }
}
impl From<(u32, u32)> for Mode {
    fn from( (bin_config, bin_speed): (u32, u32) ) -> Mode {
        if bin_speed == 0 {
            let config: InputConfig = (bin_config, bin_speed).into();
            Mode::Input(config)
        } else {
            let config: OutputConfig = (bin_config, bin_speed).into();
            Mode::Output(config)
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
pub enum PortMaxSpeed {
    Reserved = 0,
    S2MHz = 2,
    S10MHz = 1,
    S50MHz = 3,
}
impl From<u32> for PortMaxSpeed {
    fn from(bin_speed: u32) -> PortMaxSpeed {
        match bin_speed {
            0 => PortMaxSpeed::Reserved,
            1 => PortMaxSpeed::S10MHz,
            2 => PortMaxSpeed::S2MHz,
            3 => PortMaxSpeed::S50MHz,
            _ => panic!(""),
        }
    }
}