use super::{
    register::Register,
    rcc::Rcc,
};
pub struct Afio {
    mapr: Register,
}
impl Afio {
    pub fn new() -> Afio {
        Afio::enable_clock();

        let address = 0x4001_0000;
        Afio {
            mapr: Register::new(address + 0x04),
        }
    }
    pub fn disable_jtag_enable_ports(&self) {
        self.mapr.write_or(0x0200_0000);
    }
    fn enable_clock() {
        Rcc::new().enable_afio();
    }
    fn disable_clock() {
        Rcc::new().disable_afio();
    }
}