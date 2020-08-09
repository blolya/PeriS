#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    communication::uart::usart1::Usart1,
    clock,
};

#[entry]
fn main() -> ! {
    clock::init();

    let ua1 = Usart1::new();
    ua1.send('L' as u32);
    ua1.send('o' as u32);
    ua1.send('h' as u32);
    ua1.send('\r' as u32);

    loop {}
}
