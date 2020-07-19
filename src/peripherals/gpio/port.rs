use super::Gpio;

pub struct Port<'a, T: Gpio> {
    port_num: PortNum,
    gpio: &'a T,
}
impl<'a, T: Gpio> Port<'a, T> {
    pub fn new(port_num: PortNum, port_mode: PortMode, gpio: &'a T) -> Port<'a, T> {
        let port = Port { port_num, gpio };
        port.set_mode(port_mode);
        port
    }
    pub fn set_high(&self) {
        let port_mode = self.get_mode();
        let port_num: u32 = (&self.port_num).into();
        if let PortMode::Input(_) = port_mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.set_port_output(port_num);
        }
    }
    pub fn set_low(&self) {
        let port_mode = self.get_mode();
        let port_num: u32 = (&self.port_num).into();
        if let PortMode::Input(_) = port_mode {
            panic!("Port {} is in input mode. Consider to change port mode to output.", port_num);
        } else {
            self.gpio.reset_port_output(port_num);
        }
    }
    pub fn set_mode(&self, port_mode: PortMode) {
        let (gpio_config, gpio_mode, gpio_odr) = port_mode.into();
        
        let port_num: u32 = (&self.port_num).into();
        self.gpio.set_port_config(port_num, gpio_config);
        self.gpio.set_port_mode(port_num, gpio_mode);
        self.gpio.write_port_output(port_num, gpio_odr);
    }
    pub fn get_mode(&self) -> PortMode {
        let port_num: u32 = (&self.port_num).into();

        let gpio_config = self.gpio.get_port_config(port_num);
        let gpio_mode = self.gpio.get_port_mode(port_num);
        let gpio_odr = self.gpio.get_port_output(port_num);

        let port_mode: PortMode = (gpio_config, gpio_mode, gpio_odr).into();
        port_mode
    }
}

pub enum PortMode {
    Input(InputConfig),
    Output(OutputConfig),
}
impl From<PortMode> for (u32, u32, u32) {
    fn from(port_mode: PortMode) -> (u32, u32, u32) {
        match port_mode {
            PortMode::Input(config) => {
                config.into()
            },
            PortMode::Output(config) => {
                config.into()
            }
        }
    }
}
impl From<(u32, u32, u32)> for PortMode {
    fn from( (gpio_config, gpio_mode, gpio_odr): (u32, u32, u32) ) -> PortMode {
        if gpio_mode == 0 {
            let config: InputConfig = (gpio_config, gpio_odr).into();
            PortMode::Input(config)
        } else {
            let config: OutputConfig = (gpio_config, gpio_mode, gpio_odr).into();
            PortMode::Output(config)
        }
    }
}

pub enum OutputConfig {
    GeneralPurposePushPull(MaxSpeed),
    GeneralPurposeOpenDrain(MaxSpeed),
    AlternativeFunctionPushPull(MaxSpeed),
    AlternativeFunctionOpenDrain(MaxSpeed),
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
    fn from( (gpio_config, gpio_mode, _): (u32, u32, u32) ) -> OutputConfig {
        let speed: MaxSpeed = gpio_mode.into();
        match gpio_config {
            0 => OutputConfig::GeneralPurposePushPull(speed),
            1 => OutputConfig::GeneralPurposeOpenDrain(speed),
            2 => OutputConfig::AlternativeFunctionPushPull(speed),
            3 => OutputConfig::AlternativeFunctionOpenDrain(speed),
            _ => panic!("Output config can be 0, 1, 2 or 3"),
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
impl From <(u32, u32)> for InputConfig {
    fn from( (gpio_config, gpio_odr): (u32, u32) ) -> InputConfig {
        match gpio_config {
            0 => InputConfig::Analog,
            1 => InputConfig::Floating,
            2 => {
                match gpio_odr {
                    0 => InputConfig::PullDown,
                    1 => InputConfig::PullUp,
                    _ => panic!("Gpio odr value for input config can only be 0 or 1"),
                }
            }
            _ => panic!("Input config can be 0, 1 or 2"),
        }
    }
}

pub enum MaxSpeed {
    S2MHz = 2,
    S10MHz = 1,
    S50MHz = 3,
}
impl From<u32> for MaxSpeed {
    fn from(bin_speed: u32) -> MaxSpeed {
        match bin_speed {
            1 => MaxSpeed::S10MHz,
            2 => MaxSpeed::S2MHz,
            3 => MaxSpeed::S50MHz,
            _ => panic!("Max speed can be 1, 2 or 3"),
        }
    }
}

pub enum PortNum {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}
impl From<PortNum> for u32 {
    fn from(port_num: PortNum) -> u32 {
        match port_num {
            PortNum::P0 => 0,
            PortNum::P1 => 1,
            PortNum::P2 => 2,
            PortNum::P3 => 3,
            PortNum::P4 => 4,
            PortNum::P5 => 5,
            PortNum::P6 => 6,
            PortNum::P7 => 7,
            PortNum::P8 => 8,
            PortNum::P9 => 9,
            PortNum::P10 => 10,
            PortNum::P11 => 11,
            PortNum::P12 => 12,
            PortNum::P13 => 13,
            PortNum::P14 => 14,
            PortNum::P15 => 15,
        }
    }
}
impl From<&PortNum> for u32 {
    fn from(port_num: &PortNum) -> u32 {
        match port_num {
            PortNum::P0 => 0,
            PortNum::P1 => 1,
            PortNum::P2 => 2,
            PortNum::P3 => 3,
            PortNum::P4 => 4,
            PortNum::P5 => 5,
            PortNum::P6 => 6,
            PortNum::P7 => 7,
            PortNum::P8 => 8,
            PortNum::P9 => 9,
            PortNum::P10 => 10,
            PortNum::P11 => 11,
            PortNum::P12 => 12,
            PortNum::P13 => 13,
            PortNum::P14 => 14,
            PortNum::P15 => 15,
        }
    }
}
impl From<u32> for PortNum {
    fn from(port_num: u32) -> PortNum {
        match port_num {
            0 => PortNum::P0,
            1 => PortNum::P1,
            2 => PortNum::P2,
            3 => PortNum::P3,
            4 => PortNum::P4,
            5 => PortNum::P5,
            6 => PortNum::P6,
            7 => PortNum::P7,
            8 => PortNum::P8,
            9 => PortNum::P9,
            10 => PortNum::P10,
            11 => PortNum::P11,
            12 => PortNum::P12,
            13 => PortNum::P13,
            14 => PortNum::P14,
            15 => PortNum::P15,
            _ => panic!("Port num out of bounds. Max = 15"),
        }
    }
}