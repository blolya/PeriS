#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::gpioc::Gpioc;
use peris::peripherals::gpio::Gpio;

#[entry]
fn main() -> ! {
    {
        let gpioc = Gpioc::new();
        gpioc.set_port_mode(13, 0);
        gpioc.set_port_speed(13, 2);
        gpioc.set_port_high(13);
    }
    loop {}
}
