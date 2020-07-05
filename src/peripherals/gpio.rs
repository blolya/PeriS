pub mod gpioc;

use super::Clocked;
pub trait Gpio: Clocked {
    fn set_port_high(&self, port: u32);
    fn set_port_low(&self, port: u32);
    fn set_port_mode(&self, port: u32, mode: u32);
    fn set_port_speed(&self, port: u32, speed: u32);
}
