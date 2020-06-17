#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

#[entry]
fn main() -> ! {
    loop {}
}
