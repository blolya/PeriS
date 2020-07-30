use super::super::core::Register;

pub struct Flash {
    acr: Register,
}
impl Flash {
    pub fn new() -> Flash {
        let address =  0x4002_2000;
        Flash {
            acr: Register::new(address),
        }
    }
    pub fn set_latency(&self, latency: u32) {
        self.acr.write_or( (self.acr.read() & !0x7) | latency );
    }
}