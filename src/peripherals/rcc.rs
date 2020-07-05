use super::super::core::Register;

pub struct Rcc {
    address: u32,
    apb2enr: Register,
}
impl Rcc {
    pub fn new() -> Rcc {
        let address = 0x4002_1000;
        Rcc {
            address,
            apb2enr: Register::new(address + 0x18),
        }
    }

    pub fn enable_iopc(&self) {
        self.apb2enr.write_or(0x0000_0010);
    }
    pub fn disable_iopc(&self) {
        self.apb2enr.write_and(0x1111_11E1);
    }
}
