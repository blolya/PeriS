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
    pub fn set_pll_source(&self, source: u32) {
        self.cfgr.write_bit(16, source)
    }
    pub fn get_pll_source(&self) -> u32 {
        self.cfgr.get_bit(16)
    }
    pub fn set_pll_multiplication_factor(&self, factor: u32) {
        self.cfgr.write_or(factor << 18);
    }
    pub fn get_pll_multiplication_factor(&self) -> u32 {
        let factor = (self.cfgr.read() >> 18) & 0xF;
        factor
    }

    // Gpio section
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
}


pub struct SystemClock {
    rcc: Rcc,
}
impl SystemClock {
    fn new() -> SystemClock {
        SystemClock {
            rcc: Rcc::new(),
        }
    }
    fn set_source(&self, source: SystemClockSource) {
        let bin_source: u32 = source.into();
        self.rcc.set_system_clock_source(bin_source);
    }
    fn get_source(&self) -> SystemClockSource {
        let source: SystemClockSource = self.rcc.get_system_clock_source().into();
        source
    }
}

enum SystemClockSource {
    Hsi,
    Hse(Hse),
    Pll,
}
impl From<SystemClockSource> for u32 {
    fn from(source: SystemClockSource) -> u32 {
        match source {
            SystemClockSource::Hse(_) => 1,
            _ => panic!("")
        }
    }
}
impl From<u32> for SystemClockSource {
    fn from(source: u32) -> SystemClockSource {
        SystemClockSource::Hsi
    }
}

pub trait ClockSource: Device {

}

pub struct Hse {
    rcc: Rcc,
}
impl Hse {
    fn new() -> Hse {
        Hse {
            rcc: Rcc::new(),
        }
    }
}
impl ClockSource for Hse {
    
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
pub struct Pll {
    rcc: Rcc,
}
impl Pll {
    fn new( PllConfig { pll_source, pll_multiplication_factor}: PllConfig ) -> Pll {
        let pll = Pll {
            rcc: Rcc::new(),
        };
        pll.set_pll_source(pll_source);
        pll.set_pll_multiplication_factor(pll_multiplication_factor);
        pll.enable();
        pll
    }
    fn set_pll_source(&self, source: PllSource) {
        self.rcc.set_pll_source(source as u32);
    }
    fn get_pll_source(&self) -> PllSource {
        let source: PllSource = self.rcc.get_pll_source().into();
        source
    }
    fn set_pll_multiplication_factor(&self, factor: PllMultiplicationFactor) {
        self.rcc.set_pll_multiplication_factor(factor as u32);
    }
    fn get_pll_multiplication_factor(&self) -> PllMultiplicationFactor {
        let factor: PllMultiplicationFactor = self.rcc.get_pll_multiplication_factor().into();
        factor
    }
}
impl Device for Pll {
    fn enable(&self) {
        self.rcc.enable_pll();
        let mut pll_rdy_status = self.rcc.get_pll_ready_status();
        while pll_rdy_status == 0 {
            pll_rdy_status = self.rcc.get_pll_ready_status()
        };
    }
    fn disable(&self) {
        
    }
}


struct PllConfig {
    pll_source: PllSource,
    pll_multiplication_factor: PllMultiplicationFactor,
}
enum PllSource {
    Hsi,
    Hse,
}
impl From<u32> for PllSource {
    fn from(source: u32) -> PllSource {
        match source {
            0 => PllSource::Hsi,
            1 => PllSource::Hse,
            _ => panic!(""),
        }
    }
}
enum PllMultiplicationFactor {
    Pllx9 = 7,
}
impl From<u32> for PllMultiplicationFactor {
    fn from(factor: u32) -> PllMultiplicationFactor {
        match factor {
            7 => PllMultiplicationFactor::Pllx9,
            _ => panic!(""),
        }
    }
}

// trait Clock {
//     fn enable(&self);
//     fn get_status(&self) -> ClockStatus;
// }

// struct Hse {
//     rcc: Rcc,
// }
// impl Hse {
//     fn new() -> Hse {
//         Hse {
//             rcc: Rcc::new(),
//         }
//     }
// }
// impl Clock for Hse {
//     fn enable(&self) {
//         self.rcc.enable_hse()
//     }
//     fn get_status(&self) -> ClockStatus {
//         let status: ClockStatus = self.rcc.get_hse_status().into();
//         status
//     }
// }

// enum ClockStatus {
//     NotReady,
//     Ready,
// }
// impl From<u32> for ClockStatus {
//     fn from(status: u32) -> ClockStatus {
//         match status {
//             0 => ClockStatus::NotReady,
//             1 => ClockStatus::Ready,
//             _ => panic!(""),
//         }
//     }
// }

