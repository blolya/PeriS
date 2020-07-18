use super::Gpio;

pub struct Port<'a, T: Gpio> {
    port_num: Num,
    gpio: &'a T,
}
impl<'a, T: Gpio> Port<'a, T> {
    pub fn new(port_num: Num, port_type: Type, gpio: &'a T) -> Port<'a, T> {
        let port = Port { port_num, gpio };
        port.set_port_type(port_type);
        port
    }
    pub fn set_high(&self) {
        let port_type = self.get_port_type();
        let port_num: u32 = (&self.port_num).into();
        if let Type::Input(_) = port_type {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.set_port_output(port_num);
        }
    }
    pub fn set_low(&self) {
        let port_type = self.get_port_type();
        let port_num: u32 = (&self.port_num).into();
        if let Type::Input(_) = port_type {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.reset_port_output(port_num);
        }
    }
    pub fn set_port_type(&self, port_type: Type) {
        let (bin_config, bin_speed, bin_odr) = port_type.into();
        
        let port_num: u32 = (&self.port_num).into();
        self.gpio.set_port_config(port_num, bin_config);
        self.gpio.set_port_mode(port_num, bin_speed);
        self.gpio.write_port_output(port_num, bin_odr);
    }
    pub fn get_port_type(&self) -> Type {
        let port_num: u32 = (&self.port_num).into();

        let bin_config = self.gpio.get_port_config(port_num);
        let bin_mode = self.gpio.get_port_mode(port_num);
        let bin_odr = self.gpio.get_port_output(port_num);

        let port_type: Type = (bin_config, bin_mode, bin_odr).into();
        port_type
    }
}

pub enum Type {
    Input(InputConfig),
    Output(OutputConfig),
}
impl From<Type> for (u32, u32, u32) {
    fn from(port_type: Type) -> (u32, u32, u32) {
        match port_type {
            Type::Input(config) => {
                config.into()
            },
            Type::Output(config) => {
                config.into()
            }
        }
    }
}
impl From<(u32, u32, u32)> for Type {
    fn from( (bin_config, bin_mode, bin_odr): (u32, u32, u32) ) -> Type {
        if bin_mode == 0 {
            let config: InputConfig = (bin_config, bin_mode, bin_odr).into();
            Type::Input(config)
        } else {
            let config: OutputConfig = (bin_config, bin_mode, bin_odr).into();
            Type::Output(config)
        }
    }
}

pub enum OutputConfig {
    GeneralPurposePushPull(Mode),
    GeneralPurposeOpenDrain(Mode),
    AlternativeFunctionPushPull(Mode),
    AlternativeFunctionOpenDrain(Mode),
}
impl From<OutputConfig> for (u32, u32, u32) {
    fn from(config: OutputConfig) -> (u32, u32, u32) {
        match config {
            OutputConfig::GeneralPurposePushPull(speed) => (0, speed as u32, 0),
            OutputConfig::GeneralPurposeOpenDrain(speed) => (1, speed as u32, 0),
            OutputConfig::AlternativeFunctionPushPull(speed) => (2, speed as u32, 0),
            OutputConfig::AlternativeFunctionOpenDrain(speed) => (3, speed as u32, 0),
        }
    }
}
impl From<(u32, u32, u32)> for OutputConfig {
    fn from( (bin_config, bin_mode, _): (u32, u32, u32) ) -> OutputConfig {
        let speed: Mode = bin_mode.into();
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
impl From<InputConfig> for (u32, u32, u32) {
    fn from(config: InputConfig) -> (u32, u32, u32) {
        match config {
            InputConfig::Analog => (0, 0, 0),
            InputConfig::Floating => (1, 0, 0),
            InputConfig::PullDown => (2, 0, 0),
            InputConfig::PullUp => (2, 0, 1),   
        }
    }
}
impl From <(u32, u32, u32)> for InputConfig {
    fn from( (bin_config, _, bin_odr): (u32, u32, u32) ) -> InputConfig {
        match bin_config {
            0 => InputConfig::Analog,
            1 => InputConfig::Floating,
            2 => {
                match bin_odr {
                    0 => InputConfig::PullDown,
                    1 => InputConfig::PullUp,
                    _ => panic!(""),
                }
            }
            _ => panic!("")
        }
    }
}

pub enum Mode {
    Reserved = 0,
    S2MHz = 2,
    S10MHz = 1,
    S50MHz = 3,
}
impl From<u32> for Mode {
    fn from(bin_speed: u32) -> Mode {
        match bin_speed {
            0 => Mode::Reserved,
            1 => Mode::S10MHz,
            2 => Mode::S2MHz,
            3 => Mode::S50MHz,
            _ => panic!(""),
        }
    }
}

pub enum Num {
    P13,
}
impl From<Num> for u32 {
    fn from(port_num: Num) -> u32 {
        match port_num {
            Num::P13 => 13,
            _ => panic!(""),
        }
    }
}
impl From<&Num> for u32 {
    fn from(port_num: &Num) -> u32 {
        match port_num {
            Num::P13 => 13,
            _ => panic!(""),
        }
    }
}
impl From<u32> for Num {
    fn from(port_num: u32) -> Num {
        match port_num {
            13 => Num::P13,
            _ => panic!("Hey"),
        }
    }
}