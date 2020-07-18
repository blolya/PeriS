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
        if let Mode::Input(_) = mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", PortNum::to_int(&self.port_num));
        } else {
            self.gpio.set_port_output(PortNum::to_int(&self.port_num));
        }
    }
    pub fn set_low(&self) {
        let mode = self.get_mode();
        if let Mode::Input(_) = mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", PortNum::to_int(&self.port_num));
        } else {
            self.gpio.reset_port_output(PortNum::to_int(&self.port_num));
        }
    }
    pub fn set_mode(&self, mode: Mode) {
        match mode {
            Mode::Input(config) => {
                let bin_config: u32;
                match config {
                    InputConfig::Analog => bin_config = 0,
                    InputConfig::Floating => bin_config = 1,
                    InputConfig::PullDown => bin_config = 2,
                    InputConfig::PullUp => bin_config = 2,
                }
                self.gpio.set_port_mode(PortNum::to_int(&self.port_num), 0);
                self.gpio.set_port_config(PortNum::to_int(&self.port_num), bin_config);
            },
            Mode::Output(config) => {
                let bin_speed: u32;
                let bin_config: u32;
                match config {
                    OutputConfig::GeneralPurposePushPull(speed) => {
                        bin_speed = speed;
                        bin_config = 0;
                    },
                    OutputConfig::GeneralPurposeOpenDrain(speed) => {
                        bin_speed = speed;
                        bin_config = 1;
                    },
                    OutputConfig::AlternativeFunctionPushPull(speed) => {
                        bin_speed = speed;
                        bin_config = 2;
                    },
                    OutputConfig::AlternativeFunctionOpenDrain(speed) => {
                        bin_speed = speed;
                        bin_config = 3;
                    },
                }
                self.gpio.set_port_mode(PortNum::to_int(&self.port_num), bin_speed);
                self.gpio.set_port_config(PortNum::to_int(&self.port_num), bin_config);
            },
        }
    }
    pub fn get_mode(&self) -> Mode {
        let bin_config = self.gpio.get_port_config(PortNum::to_int(&self.port_num));
        let bin_mode = self.gpio.get_port_mode(PortNum::to_int(&self.port_num));

        match bin_mode {
            0 => {
                match bin_config {
                    0 => Mode::Input(InputConfig::Analog),
                    1 => Mode::Input(InputConfig::Floating),
                    2 => Mode::Input(InputConfig::PullDown),
                    _ => Mode::Input(InputConfig::PullUp)
                }
            },
            speed => {
                match bin_config {
                    0 => Mode::Output(OutputConfig::GeneralPurposePushPull(speed)),
                    1 => Mode::Output(OutputConfig::GeneralPurposeOpenDrain(speed)),
                    2 => Mode::Output(OutputConfig::AlternativeFunctionPushPull(speed)),
                    _ => Mode::Output(OutputConfig::AlternativeFunctionOpenDrain(speed)),
                }
            },
        }
    }
}

pub enum OutputConfig {
    GeneralPurposePushPull(u32),
    GeneralPurposeOpenDrain(u32),
    AlternativeFunctionPushPull(u32),
    AlternativeFunctionOpenDrain(u32),
}
pub enum InputConfig {
    Analog,
    Floating,
    PullDown,
    PullUp,
}
pub enum Mode {
    Input(InputConfig),
    Output(OutputConfig),
}

pub enum PortNum {
    P13,
}
impl PortNum {
    fn from_int(port_num: u32) -> PortNum {
        match port_num {
            13 => PortNum::P13,
            _ => panic!("Port not found"),
        }
    }
    fn to_int(port_num: &PortNum) -> u32 {
        match port_num {
            PortNum::P13 => 13,
            _ => panic!("Port not found"),
        }
    }
}