pub mod gpioa;
pub mod gpiob;
pub mod gpioc;

use super::register::Register;

pub struct Gpio {
    crl: Register,
    crh: Register,
    idr: Register,
    odr: Register,
    pub bsrr: Register,
    brr: Register,
    lckr: Register,
}
impl Gpio {
    pub fn set_port_config(&self, port: u32, config: u32) {
        let config = config & 0x0000_0003;
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        cr.write_and(!(0b11 << 2 + shift_num * 4));
        cr.write_or(config << 2 + shift_num * 4);
    }
    pub fn get_port_config(&self, port: u32) -> u32 {
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        let config = cr.read() & 0b11 << 2 + shift_num * 4;
        config >> 2 + shift_num * 4
    }
    pub fn set_port_mode(&self, port: u32, mode: u32) {
        let mode = mode & 0x0000_0003;
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        cr.write_and(!(0b11 << shift_num * 4));
        cr.write_or(mode << shift_num * 4);
    }
    pub fn get_port_mode(&self, port: u32) -> u32 {
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        let mode = cr.read() & 0b11 << shift_num * 4;
        mode >> shift_num * 4
    }
    pub fn set_port_output(&self, port: u32) {
        self.odr.set_bit(port);
    }
    pub fn write_port_output(&self, port: u32, value: u32) {
        self.odr.write_bit(port, value);
    }
    pub fn reset_port_output(&self, port: u32) {
        self.brr.set_bit(port);
    }
    pub fn get_port_output(&self, port: u32) -> u32 {
        self.odr.get_bit(port)
    }
    pub fn get_port_input(&self, port: u32) -> u32 {
        self.idr.get_bit(port)
    }
    pub fn select_port_cr_and_shift_num(&self, port: u32) -> (&Register, u32) {
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        }
    }
}