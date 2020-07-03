pub mod gpio;
pub mod rcc;

use gpio::Gpio;
use rcc::Rcc;

pub struct Peripherals {
    pub rcc: Rcc,
    pub gpioc: Gpio,
}
impl Peripherals {
    fn new(rcc: Rcc, gpioc: Gpio) -> Peripherals {
        Peripherals { rcc, gpioc }
    }
}

pub fn take() -> Peripherals {
    let rcc = Rcc::new();
    let gpioc = Gpio::new(0x4001_1000);
    Peripherals::new(rcc, gpioc)
}
