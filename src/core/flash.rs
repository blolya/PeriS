use super::register::Register;

pub struct Flash {
    acr: Register,
    keyr: Register,
    optkeyr: Register,
    sr: Register,
    cr: Register,
    ar: Register,
    obr: Register,
    wrpr: Register,
}
impl Flash {
    pub fn new() -> Flash {
        let address =  0x4002_2000;
        Flash {
            acr: Register::new(address),
            keyr: Register::new(address + 0x04),
            optkeyr: Register::new(address + 0x08),
            sr: Register::new(address + 0x0c),
            cr: Register::new(address + 0x10),
            ar: Register::new(address + 0x14),
            obr: Register::new(address + 0x1c),
            wrpr: Register::new(address + 0x20),
        }
    }
    pub fn start_erase(&self) {
        self.cr.set_bit(6);
    }
    pub fn set_latency(&self, latency: u32) {
        self.acr.write_or( (self.acr.read() & !0x7) | latency );
    }
    pub fn get_cr_lock(&self) -> u32 {
        self.cr.get_bit(7)
    }
    pub fn unlock_cr(&self) {
        self.keyr.write(0x45670123);
        self.keyr.write(0xcdef89ab);
    }
    pub fn set_address(&self, address: u32) {
        self.ar.write(address);
    }
    pub fn get_sr_bsy(&self) -> u32 {
        self.sr.get_bit(0)
    }
    pub fn get_eop(&self) -> u32 {
        self.sr.get_bit(5)
    }
    pub fn reset_eop(&self) {
        self.sr.set_bit(5);
    }
    pub fn select_page_erase(&self) {
        self.cr.set_bit(1);
    }
    pub fn unselect_page_erase(&self) {
        self.cr.reset_bit(1);
    }
    pub fn select_programming(&self) {
        self.cr.set_bit(0);
    }
    pub fn unselect_programming(&self) {
        self.cr.reset_bit(0);
    }
}