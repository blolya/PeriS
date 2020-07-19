#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::{port::PortNum, port::MaxSpeed, port::Port, port::PortMode, port::OutputConfig, gpioc::Gpioc};


#[entry]
fn main() -> ! {
    let gpioc = Gpioc::new();
    let mode = PortMode::Output(OutputConfig::GeneralPurposePushPull( MaxSpeed::S2MHz ));
    let p13 = Port::new(PortNum::P13, mode, &gpioc);
    p13.set_high();

    loop {}
}
