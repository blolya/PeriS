use super::{
    Gpio,
    super::{
        register::Register,
        rcc::Rcc
    },
};
pub struct Gpioc;
impl Gpioc {
    pub fn new() -> Gpio {
        Gpioc::enable_clock();
        let address = 0x4001_1000;
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
        Rcc::new().enable_iopc();
    }
    fn disable_clock() {
        Rcc::new().disable_iopc();
    }
}