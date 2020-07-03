pub mod gpio;

use super::core::Register;

pub struct Peripherals {
    pub rcc: Rcc,
    pub gpioc: Gpio,
}
impl Peripherals {
    fn new(rcc: Rcc, gpioc: Gpio) -> Peripherals {
        Peripherals { rcc, gpioc }
    }
}

pub struct Rcc {
    address: u32,
    apb2enr: Register,
}
impl Rcc {
    pub fn new() -> Rcc {
        let address = 0x4002_1000;
        Rcc {
            address,
            apb2enr: Register::new(address + 0x18),
        }
    }

    pub fn enable_iopc(&self) {
        self.apb2enr.write(0x0000_0010);
    }
}

pub struct Gpio {
    address: u32,
    pub crl: Register,
    pub crh: Register,
    pub bsrr: Register,
}
impl Gpio {
    fn new(address: u32) -> Gpio {
        Gpio {
            address,
            crl: Register::new(address),
            crh: Register::new(address + 0x04),
            bsrr: Register::new(address + 0x10),
        }
    }
}

pub fn take() -> Peripherals {
    let rcc = Rcc::new();
    let gpioc = Gpio::new(0x4001_1000);
    Peripherals::new(rcc, gpioc)
}
