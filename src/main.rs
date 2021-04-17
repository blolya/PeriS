#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    fprom::Fprom,
    communication::uart::usart1::Usart1,
    clock,
    ports::{
        Port,
        PortMode,
        PortNum,
        MaxSpeed,
        OutputConfig
    },
};

use peris::core::gpio::gpioc::Gpioc;
use peris::core::flash::Flash;

#[entry]
fn main() -> ! {
    clock::init();

    let mut fprom = Fprom::new(0x0000_f800);

    fprom.erase();

    loop {}
}
