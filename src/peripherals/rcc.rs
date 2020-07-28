pub mod clock;

use super::super::core::Register;

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
        self.cfgr.write_or( (self.cfgr.read() & !0x3) | system_clock );
    }
    pub fn get_system_clock_source(&self) -> u32 {
        let status = self.cfgr.read();
        (status & 0xC) >> 2
    }

    // Hsi 
    pub fn enable_hsi(&self) {
        self.cr.set_bit(0);
    }
    pub fn disable_hsi(&self) {
        self.cr.reset_bit(0);
    }
    pub fn get_hsi_ready_status(&self) -> u32 {
        self.cr.get_bit(1)
    }

    // Hse
    pub fn enable_hse(&self) {
        self.cr.set_bit(16);
    }
    pub fn disable_hse(&self) {
        self.cr.reset_bit(16);
    }
    pub fn get_hse_ready_status(&self) -> u32 {
        self.cr.get_bit(17)
    }

    // Pll
    pub fn enable_pll(&self) {
        self.cr.set_bit(24);
    }
    pub fn disable_pll(&self) {
        self.cr.reset_bit(24);
    }
    pub fn get_pll_ready_status(&self) -> u32 {
        self.cr.get_bit(25)
    }
    pub fn set_pll_clock_source(&self, source: u32) {
        self.cfgr.write_bit(16, source);
    }
    pub fn get_pll_clock_source(&self) -> u32 {
        self.cfgr.get_bit(16)
    }
    pub fn set_pll_multiplication_factor(&self, factor: u32) {
        self.cfgr.write( (self.cfgr.read() & !(0xF << 18)) | (factor << 18) );
    }
    pub fn get_pll_multiplication_factor(&self) -> u32 {
        let factor = (self.cfgr.read() >> 18) & 0xF;
        factor
    }
    pub fn set_pll_prescaler(&self, prescaler: u32) {
        self.cfgr.write_bit(17, prescaler);
    }
    pub fn get_pll_prescaler(&self) -> u32 {
        self.cfgr.get_bit(17)
    }

    // Prescalers 
    pub fn set_ahb_prescaler(&self, prescaler: u32) {
        self.cfgr.write( (self.cfgr.read() & !(0xF << 4)) | (prescaler << 4) );
    }
    pub fn get_ahb_prescaler(&self) -> u32 {
        let prescaler = (self.cfgr.read() >> 4) & 0xF;
        prescaler
    }
    pub fn set_apb1_prescaler(&self, prescaler: u32) {
        self.cfgr.write( (self.cfgr.read() & !(0x7 << 8)) | (prescaler << 8) );
    }
    pub fn get_apb1_prescaler(&self) -> u32 {
        let prescaler = (self.cfgr.read() >> 8) & 0x7;
        prescaler
    }
    pub fn set_apb2_prescaler(&self, prescaler: u32) {
        self.cfgr.write( (self.cfgr.read() & !(0x7 << 11)) | (prescaler << 11) );
    }
    pub fn get_apb2_prescaler(&self) -> u32 {
        let prescaler = (self.cfgr.read() >> 11) & 0x7;
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

