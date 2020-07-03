#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio;

#[entry]
fn main() -> ! {
    let pc13 = gpio::gpioc().pc13;
    pc13.set_high();
    loop {}
}
