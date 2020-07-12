pub mod gpioc;

use super::Clocked;
pub trait Gpio: Clocked {
    fn set_port_high(&self, port: u32);
    fn set_port_low(&self, port: u32);
    fn set_port_mode(&self, port: u32, mode: u32);
    fn get_port_mode(&self, port: u32) -> u32;
    fn set_port_config(&self, port: u32, config: u32);
    fn get_port_config(&self, port: u32) -> u32;
}

pub struct Port<'a, T: Gpio> {
    port_num: u32,
    gpio: &'a T,
}
impl<'a, T: Gpio> Port<'a, T> {
    pub fn new(port_num: u32, gpio: &'a T, mode: Mode) -> Port<'a, T> {
        let port = Port { port_num, gpio };
        port.set_mode(mode);
        port
    }
    pub fn set_high(&self) -> Result<(), Errq> {
        let mode = self.get_mode();
        if let Mode::Input(_) = mode {
            Err(Errq)
        } else {
            self.gpio.set_port_high(self.port_num);
            Ok(())
        }
    }
    pub fn set_low(&self) -> Result<(), Errq> {
        let mode = self.get_mode();
        if let Mode::Input(_) = mode {
            Err(Errq)
        } else {
            self.gpio.set_port_low(self.port_num);
            Ok(())
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
                self.gpio.set_port_mode(self.port_num, 0);
                self.gpio.set_port_config(self.port_num, bin_config);
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
                self.gpio.set_port_mode(self.port_num, bin_speed);
                self.gpio.set_port_config(self.port_num, bin_config);
            },
        }
    }
    pub fn get_mode(&self) -> Mode {
        let bin_config = self.gpio.get_port_config(self.port_num);
        let bin_mode = self.gpio.get_port_mode(self.port_num);

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
pub struct Errq;