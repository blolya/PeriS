use crate::core::rcc::Rcc;
use crate::core::register::Register;
pub struct Usb {
    pub ep0r: Register,
    pub ep1r: Register,
    pub ep2r: Register,
    pub cntr: Register,
    pub istr: Register,
    pub fnr: Register,
    pub daddr: Register,
    pub btable: Register,
}
impl Usb {
    pub fn new() -> Usb {
        let address = 0x4000_5C00;
        Usb {
            ep0r: Register::new(address),
            ep1r: Register::new(address + 0x04),
            ep2r: Register::new(address + 0x08),
            cntr: Register::new(address + 0x40),
            istr: Register::new(address + 0x44),
            fnr: Register::new(address + 0x48),
            daddr: Register::new(address + 0x4C),
            btable: Register::new(address + 0x50),
        }
    }
}