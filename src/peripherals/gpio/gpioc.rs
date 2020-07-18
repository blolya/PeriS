use super::super::super::core::Register;
use super::super::rcc::Rcc;
use super::super::Clocked;
use super::Gpio;
pub struct Gpioc {
    crl: Register,
    crh: Register,
    idr: Register,
    odr: Register,
    bsrr: Register,
    brr: Register,
    lckr: Register,
}
impl Gpioc {
    pub fn new() -> Gpioc {
        Gpioc::enable_clock();
        let address = 0x4001_1000;
        Gpioc {
            crl: Register::new(address),
            crh: Register::new(address + 0x04),
            idr: Register::new(address + 0x08),
            odr: Register::new(address + 0x0C),
            bsrr: Register::new(address + 0x10),
            brr: Register::new(address + 0x14),
            lckr: Register::new(address + 0x18),         
        }
    }
}

impl Gpio for Gpioc {
    fn set_port_mode(&self, port: u32, mode: u32) {
        let (cr, shift_num) = 
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        };
        cr.write_and(!(0b11 << shift_num * 4));
        cr.write_or(mode << shift_num * 4);
    }
    fn get_port_mode(&self, port: u32) -> u32 {
        let (cr, shift_num) = 
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        };
        let mode = cr.read() & 0b11 << shift_num * 4;
        mode >> shift_num * 4
    }
    fn set_port_config(&self, port: u32, config: u32) {
        let (cr, shift_num) = 
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        };
        cr.write_and(!(0b11 << 2 + shift_num * 4));
        cr.write_or(config << 2 + shift_num * 4);
    }
    fn get_port_config(&self, port: u32) -> u32 {
        let (cr, shift_num) = 
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        };
        let config = cr.read() & 0b11 << 2 + shift_num * 4;
        config >> 2 + shift_num * 4
    }
    fn set_port_output(&self, port: u32) {
        self.odr.set_bit(port);
    }
    fn write_port_output(&self, port: u32, value: u32) {
        self.odr.write_bit(port, value);
    }
    fn reset_port_output(&self, port: u32) {
        self.brr.set_bit(port);
    }
    fn get_port_output(&self, port: u32) -> u32 {
        self.odr.get_bit(port)
    }
    fn get_port_input(&self, port: u32) -> u32 {
        self.idr.get_bit(port)
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
