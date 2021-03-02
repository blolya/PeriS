use crate::core::register::Register;

pub struct Nvic {
    pub iser0: Register,
}
impl Nvic {
    pub fn new() -> Nvic {
        let address: u32 = 0xe000_e100;

        Nvic {
            iser0: Register::new(address),
        }
    }
}