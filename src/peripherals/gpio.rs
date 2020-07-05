use super::super::core::Register;
use super::rcc::Rcc;

pub enum PortType {
    A,
    B,
    C,
    D,
    E,
}

struct Gpioc;
impl Clocked for Gpioc {
    fn enable_clock(&self) {
        Rcc::new().enable_iopc();
    }
    fn disable_clock(&self) {
        Rcc::new().disable_iopc();
    }
}

pub trait Clocked {
    fn enable_clock(&self);
    fn disable_clock(&self);
}

pub struct Gpio {
    address: u32,
    pub crl: Register,
    pub crh: Register,
    pub bsrr: Register,
}
impl Gpio {
    pub fn new(address: u32, port_type: impl Clocked) -> Gpio {
        port_type.enable_clock();
        Gpio {
            address,
            crl: Register::new(address),
            crh: Register::new(address + 0x04),
            bsrr: Register::new(address + 0x10),
        }
    }
}

pub struct Port {
    port_num: u32,
    gpio: Gpio,
}
impl Port {
    pub fn new(port_num: u32, gpio: Gpio) -> Port {
        Port { port_num, gpio }
    }
    pub fn set_high(&self) {
        self.gpio.bsrr.write(0b1 << self.port_num);
    }
    pub fn set_low(&self) {
        self.gpio.bsrr.write(0b1 << self.port_num + 16);
    }
    pub fn set_mode(&self, mode: u32) {
        let cr;
        let shift_num;
        if self.port_num > 7 {
            cr = &self.gpio.crh;
            shift_num = self.port_num - 8;
        } else {
            cr = &self.gpio.crl;
            shift_num = self.port_num;
        };

        cr.write_and(!(0b11 << 2 + shift_num * 4));
        cr.write_or(mode << 2 + shift_num * 4);
    }
    pub fn set_speed(&self, speed: u32) {
        let cr;
        let shift_num;
        if self.port_num > 7 {
            cr = &self.gpio.crh;
            shift_num = self.port_num - 8;
        } else {
            cr = &self.gpio.crl;
            shift_num = self.port_num;
        };

        cr.write_and(!(0b11 << shift_num * 4));
        cr.write_or(speed << shift_num * 4);
    }
}
impl Drop for Port {
    fn drop(&mut self) {
        self.set_low();
        self.set_mode(0);
        self.set_speed(0);
    }
}
pub struct Ports {
    pub p13: Port,
}

pub fn gpioc() -> Ports {
    let gpioc = Gpio::new(0x4001_1000, Gpioc);

    let p13 = Port::new(13, gpioc);
    Ports { p13 }
}
