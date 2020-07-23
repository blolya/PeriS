use super::super::core::Register;
use super::{Device, Clocked};

pub struct Rcc {
    cr: Register,
    cfgr: Register,
    apb2enr: Register,
}
impl Rcc {
    pub fn new() -> Rcc {
        let address = 0x4002_1000;
        Rcc {
            cr: Register::new(address),
            cfgr: Register::new(address + 0x04),
            apb2enr: Register::new(address + 0x18),
        }
    }

    // Clock section
    pub fn set_system_clock_source(&self, system_clock: u32) {
        self.cfgr.write_and(!0x0000_0003);
        self.cfgr.write_or(system_clock);
    }
    pub fn get_system_clock_source(&self) -> u32 {
        let status = self.cfgr.read();
        (status & 0x0000_000C) >> 2
    }

    pub fn enable_hse(&self) {
        self.cr.set_bit(16);
    }
    pub fn get_hse_ready_status(&self) -> u32 {
        self.cr.get_bit(17)
    }

    pub fn enable_pll(&self) {
        self.cr.set_bit(24)
    }
    pub fn get_pll_ready_status(&self) -> u32 {
        self.cr.get_bit(25)
    }
    pub fn set_pll_source_clock(&self, source: u32) {
        self.cfgr.write_bit(16, source)
    }
    pub fn get_pll_source_clock(&self) -> u32 {
        self.cfgr.get_bit(16)
    }
    pub fn set_pll_multiplication_factor(&self, factor: u32) {
        self.cfgr.write_and( !0x003C_0000 );
        self.cfgr.write_or(factor << 18);
    }
    pub fn get_pll_multiplication_factor(&self) -> u32 {
        let factor = (self.cfgr.read() >> 18) & 0xF;
        factor
    }
    pub fn set_apb1_prescaler(&self, prescaler: u32) {
        self.cfgr.write_and(!0x0000_0700);
        self.cfgr.write_or(prescaler << 8);
    }
    pub fn get_apb1_prescaler(&self) -> u32 {
        let prescaler = (self.cfgr.read() >> 8) & 0x7;
        prescaler
    }

    // Gpio section
    pub fn enable_afio(&self) {
        self.apb2enr.set_bit(0);
    }
    pub fn disable_afio(&self) {
        self.apb2enr.reset_bit(0);
    }
    pub fn enable_iopa(&self) {
        self.apb2enr.write_or(0x0000_0004);
    }
    pub fn disable_iopa(&self) {
        self.apb2enr.write_and(0xFFFF_FFFD);
    }
    pub fn enable_iopb(&self) {
        self.apb2enr.write_or(0x0000_0008);
    }
    pub fn disable_iopb(&self) {
        self.apb2enr.write_and(0xFFFF_FFF7);
    }
    pub fn enable_iopc(&self) {
        self.apb2enr.write_or(0x0000_0010);
    }
    pub fn disable_iopc(&self) {
        self.apb2enr.write_and(0xFFFF_FFEF);
    }

    // Communication section
    pub fn enable_usart1(&self) {
        self.apb2enr.set_bit(14);
    }
    pub fn disable_usart1(&self) {
        self.apb2enr.reset_bit(14);
    }
}


pub struct SystemClock {
    rcc: Rcc,
}
impl SystemClock {
    pub fn new() -> SystemClock {
        SystemClock {
            rcc: Rcc::new(),
        }
    }
    pub fn set_source(&self, source: impl ClockSource) {
        source.enable();
        let bin_source: u32 = source.into();
        let flash_register = Register::new(0x4002_2000);
        flash_register.write_or(0x0000_0002);
        self.rcc.set_system_clock_source(bin_source);
    }
    pub fn get_source(&self) -> impl ClockSource {
        Hse::new()
    }
    pub fn set_apb1_prescaler(&self, prescaler: Apb1Prescaler) {
        self.rcc.set_apb1_prescaler(prescaler as u32);
    }
    pub fn get_apb1_prescaler(&self) -> Apb1Prescaler {
        let prescaler: Apb1Prescaler = self.rcc.get_apb1_prescaler().into();
        prescaler
    }
}


pub struct Hse {
    rcc: Rcc,
}
impl Hse {
    pub fn new() -> Hse {
        Hse {
            rcc: Rcc::new(),
        }
    }
}
impl ClockSource for Hse {

}
impl PllClockSource for Hse {

}
impl Device for Hse {
    fn enable(&self) {
        self.rcc.enable_hse();
        let mut hse_ready_status = self.rcc.get_hse_ready_status();
        while hse_ready_status == 0 {
            hse_ready_status = self.rcc.get_hse_ready_status();
        }
    } 
    fn disable(&self) {
        
    }
}
impl From<Hse> for u32 {
    fn from(source: Hse) -> u32 {
        1
    }
}

pub struct Pll {
    rcc: Rcc,
}
impl Pll {
    pub fn new() -> Pll {
        Pll {
            rcc: Rcc::new(),
        }
    }
    pub fn set_source(&self, source: impl PllClockSource) {
        source.enable();
        let bin_source: u32 = source.into();
        self.rcc.set_pll_source_clock(bin_source)
    }
    pub fn get_source(&self) -> impl PllClockSource {
        Hse::new()
    }
    pub fn set_multiplication_factor(&self, factor: PllMul) {
        self.rcc.set_pll_multiplication_factor(factor as u32);
    }
    pub fn get_pll_multiplication_factor(&self) -> PllMul {
        let factor: PllMul = self.rcc.get_pll_multiplication_factor().into();
        factor
    }
}
impl ClockSource for Pll {

}
impl Device for Pll {
    fn enable(&self) {
        self.rcc.enable_pll();
        let mut pll_ready_status = self.rcc.get_pll_ready_status();
        while pll_ready_status == 0 {
            pll_ready_status = self.rcc.get_pll_ready_status();
        }
    } 
    fn disable(&self) {
        
    }
}
impl From<Pll> for u32 {
    fn from(source: Pll) -> u32 {
        2
    }
}

pub trait ClockSource: Device + Into<u32> {

}
pub trait PllClockSource: Device + Into<u32> {

}

pub enum PllMul {
    Pllx9 = 7,
}
impl From<u32> for PllMul {
    fn from(mul: u32) -> PllMul {
        match mul {
            7 => PllMul::Pllx9,
            _ => panic!(""),
        }
    }
}

pub enum Apb1Prescaler {
    Db2 = 4,
}
impl From<u32> for Apb1Prescaler {
    fn from(prescaler: u32) -> Apb1Prescaler {
        match prescaler {
            4 => Apb1Prescaler::Db2,
            _ => panic!(""),
        }
    }
}