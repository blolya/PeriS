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

#[entry]
fn main() -> ! {
    clock::init();

    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output( OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz) ), &gpioc);
    pc13.set_high();
    // let mut flash = Flash::new();
    // // flash.write(0x0001_8314, &[0x1111, 0x2222, 0x3333, 0x4444, 0x0000]);
    let mut buffer: [u16; 2] = [0x0000, 0x0000];
    // flash.read(0x0001_830a, &mut buffer);

    // if buffer[2] == 0x3323 {
    //     pc13.set_low();
    // } else {
    //     pc13.set_high();
    // }

    let mut fprom = Fprom::new(0x0001_8300);
    fprom.read(&mut buffer);

    if buffer[1] == 0xcccc {

        pc13.set_low();
    }
    else {
        pc13.set_high();
    }
    loop {}
}
