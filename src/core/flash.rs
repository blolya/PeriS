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
    pub fn set_latency(&self, latency: u32) {
        self.acr.write_or( (self.acr.read() & !0x7) | latency );
    }
}