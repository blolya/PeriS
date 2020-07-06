pub mod gpioc;

use super::Clocked;
pub trait Gpio: Clocked {
    fn set_port_high(&self, port: u32);
    fn set_port_low(&self, port: u32);
    fn set_port_mode(&self, port: u32, mode: u32);
    fn set_port_speed(&self, port: u32, speed: u32);
}

pub struct Port<'a, T: Gpio> {
    port_num: u32,
    gpio: &'a T,
}
impl<'a, T: Gpio> Port<'a, T> {
    pub fn new(port_num: u32, gpio: &'a T) -> Port<'a, T> {
        Port { port_num, gpio }
    }
    pub fn set_high(&self) {
        self.gpio.set_port_high(self.port_num);
    }
    pub fn set_low(&self) {
        self.gpio.set_port_low(self.port_num);
    }
    pub fn set_mode(&self, mode: u32) {
        self.gpio.set_port_mode(self.port_num, mode);
    }
    pub fn set_speed(&self, speed: u32) {
        self.gpio.set_port_speed(self.port_num, speed);
    }
}
