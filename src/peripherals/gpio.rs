use super::{Gpio, Peripherals};

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

pub struct Ports {
    pub pc13: Port,
}

pub fn gpioc() -> Ports {
    let Peripherals { rcc, gpioc } = super::take();

    rcc.enable_iopc();

    let pc13 = Port::new(13, gpioc);
    Ports { pc13 }
}
