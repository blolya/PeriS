#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::gpioc::Gpioc;
use peris::peripherals::gpio::Port;

#[entry]
fn main() -> ! {
    let gpioc = Gpioc::new();
    let pc13 = Port::new(13, &gpioc);
    pc13.set_mode(0);
    pc13.set_speed(2);
    pc13.set_high();
    loop {}
}
