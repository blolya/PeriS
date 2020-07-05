use super::super::super::core::Register;
use super::super::rcc::Rcc;
use super::super::Clocked;
use super::Gpio;

pub struct Gpioc {
    crl: Register,
    crh: Register,
    bsrr: Register,
}
impl Gpioc {
    pub fn new() -> Self {
        Self::enable_clock();
        let address = 0x4001_1000;
        Gpioc {
            crl: Register::new(address),
            crh: Register::new(address + 0x04),
            bsrr: Register::new(address + 0x10),
        }
    }
}
impl Drop for Gpioc {
    fn drop(&mut self) {
        Self::disable_clock();
    }
}

impl Gpio for Gpioc {
    fn set_port_high(&self, port: u32) {
        self.bsrr.write(0b1 << port);
    }
    fn set_port_low(&self, port: u32) {
        self.bsrr.write(0b1 << port + 16);
    }
    fn set_port_mode(&self, port: u32, mode: u32) {
        let cr;
        let shift_num;
        if port > 7 {
            cr = &self.crh;
            shift_num = port - 8;
        } else {
            cr = &self.crl;
            shift_num = port;
        };

        cr.write_and(!(0b11 << 2 + shift_num * 4));
        cr.write_or(mode << 2 + shift_num * 4);
    }
    fn set_port_speed(&self, port: u32, speed: u32) {
        let cr;
        let shift_num;
        if port > 7 {
            cr = &self.crh;
            shift_num = port - 8;
        } else {
            cr = &self.crl;
            shift_num = port;
        };

        cr.write_and(!(0b11 << shift_num * 4));
        cr.write_or(speed << shift_num * 4);
    }
}

impl Clocked for Gpioc {
    fn enable_clock() {
        Rcc::new().enable_iopc();
    }
    fn disable_clock() {
        Rcc::new().disable_iopc();
    }
}
