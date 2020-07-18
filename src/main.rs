#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::gpio::{PortNum, PortMaxSpeed, Port, Mode, OutputConfig, gpioc::Gpioc};


#[entry]
fn main() -> ! {
    let gpioc = Gpioc::new();
    let mode = Mode::Output(OutputConfig::GeneralPurposePushPull( PortMaxSpeed::S2MHz ));
    let p13 = Port::new(PortNum::P13, &gpioc, mode);
    p13.set_high();

    loop {}
}
