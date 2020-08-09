use super::{
    Gpio,
    super::{
        register::Register,
        rcc::Rcc
    },
};
pub struct Gpioa;
impl Gpioa {
    pub fn new() -> Gpio {
        Gpioa::enable_clock();
        let address = 0x4001_0800;
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
        Rcc::new().enable_iopa();
    }
    fn disable_clock() {
        Rcc::new().disable_iopa();
    }
}
