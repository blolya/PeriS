#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::{port::Num, port::MaxSpeed, port::Port, port::Type, port::OutputConfig, gpioc::Gpioc};


#[entry]
fn main() -> ! {
    let gpioc = Gpioc::new();
    let mode = Type::Output(OutputConfig::GeneralPurposePushPull( MaxSpeed::S2MHz ));
    let p13 = Port::new(Num::P13, mode, &gpioc);
    p13.set_high();

    loop {}
}
