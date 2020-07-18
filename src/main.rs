#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::{PortNum, PortMode, Port, PortType, OutputConfig, gpioc::Gpioc};


#[entry]
fn main() -> ! {
    let gpioc = Gpioc::new();
    let mode = PortType::Output(OutputConfig::GeneralPurposePushPull( PortMode::S2MHz ));
    let p13 = Port::new(PortNum::P13, mode, &gpioc);
    p13.set_high();

    loop {}
}
