#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio;

#[entry]
fn main() -> ! {
    let pc = gpio::gpioc();
    let pc13 = &pc.p13;
    pc13.set_mode(0);
    pc13.set_speed(2);
    pc13.set_high();

    loop {}
}
