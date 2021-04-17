#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::clock;

#[entry]
fn main() -> ! {
    clock::init();

    loop {}
}
