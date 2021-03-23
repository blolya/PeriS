#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    flash::Flash,
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

#[entry]
fn main() -> ! {
    clock::init();

    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output( OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz) ), &gpioc);

    let mut flash = Flash::new();
    flash.write(0x0001_8308, &[0x5555]);


    pc13.set_low();


    loop {}
}
