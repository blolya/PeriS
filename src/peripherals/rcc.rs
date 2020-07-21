use super::super::core::Register;

pub struct Rcc {
    pub cr: Register,
    pub cfgr: Register,
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
