pub mod gpioa;
pub mod gpiob;
pub mod gpioc;
pub mod port;

use super::Clocked;
use super::super::core::Register;
pub trait Gpio: Clocked {
    fn set_port_config(&self, port: u32, config: u32);
    fn get_port_config(&self, port: u32) -> u32;
    fn set_port_mode(&self, port: u32, mode: u32);
    fn get_port_mode(&self, port: u32) -> u32;
    fn set_port_output(&self, port: u32);
    fn write_port_output(&self, port: u32, value: u32);
    fn reset_port_output(&self, port: u32);
    fn get_port_output(&self, port: u32) -> u32;
    fn get_port_input(&self, port: u32) -> u32;
    fn select_port_cr_and_shift_num(&self, port: u32) -> (&Register, u32);
}