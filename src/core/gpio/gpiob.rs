use super::{
    Gpio,
    super::{
        register::Register,
        rcc::Rcc
    },
};
pub struct Gpiob;
impl Gpiob {
    pub fn new() -> Gpio {
        Gpiob::enable_clock();
        let address = 0x4001_0C00;
        Gpio {
            crl: Register::new(address),
            crh: Register::new(address + 0x04),
            idr: Register::new(address + 0x08),
            odr: Register::new(address + 0x0C),
            bsrr: Register::new(address + 0x10),
            brr: Register::new(address + 0x14),
            lckr: Register::new(address + 0x18),         
        }
    }
    fn enable_clock() {
        Rcc::new().enable_iopb();
    }
    fn disable_clock() {
        Rcc::new().disable_iopb();
    }
}