use super::super::super::core::Register;
use super::super::rcc::Rcc;
use super::super::Clocked;
use super::Gpio;
pub struct Gpiob {
    crl: Register,
    crh: Register,
    idr: Register,
    odr: Register,
    bsrr: Register,
    brr: Register,
    lckr: Register,
}
impl Gpiob {
    pub fn new() -> Gpiob {
        Gpiob::enable_clock();
        let address = 0x4001_0C00;
        Gpiob {
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

impl Gpio for Gpiob {
    fn set_port_config(&self, port: u32, config: u32) {
        let config = config & 0x0000_0003;
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        cr.write_and(!(0b11 << 2 + shift_num * 4));
        cr.write_or(config << 2 + shift_num * 4);
    }
    fn get_port_config(&self, port: u32) -> u32 {
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        let config = cr.read() & 0b11 << 2 + shift_num * 4;
        config >> 2 + shift_num * 4
    }
    fn set_port_mode(&self, port: u32, mode: u32) {
        let mode = mode & 0x0000_0003;
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        cr.write_and(!(0b11 << shift_num * 4));
        cr.write_or(mode << shift_num * 4);
    }
    fn get_port_mode(&self, port: u32) -> u32 {
        let (cr, shift_num) = self.select_port_cr_and_shift_num(port);
        let mode = cr.read() & 0b11 << shift_num * 4;
        mode >> shift_num * 4
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
    fn select_port_cr_and_shift_num(&self, port: u32) -> (&Register, u32) {
        if port > 7 { 
            (&self.crh, port - 8) 
        } 
        else { 
            (&self.crl, port) 
        }
    }
}

impl Clocked for Gpiob {
    fn enable_clock() {
        Rcc::new().enable_iopb();
    }
    fn disable_clock() {
        Rcc::new().disable_iopb();
    }
}
