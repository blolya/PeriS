use crate::core::register::Register;

pub struct Exti {
    imr: Register,
    rtsr: Register,
    ftsr: Register,
}
impl Exti {
    pub fn new() -> Exti {
        let address: u32 = 0x4001_0400;

        Exti {
            imr: Register::new(address),
            rtsr: Register::new(address + 0x08),
            ftsr: Register::new(address + 0x0c),
        }
    }
}