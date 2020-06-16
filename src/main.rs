#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_persist as _;

#[entry]
fn main() -> ! {
    loop {}
}
