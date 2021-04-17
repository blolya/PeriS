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

    // let gpioc = Gpioc::new();
    // let pc13 = Port::new(PortNum::P13, PortMode::Output( OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz) ), &gpioc);
    // pc13.set_high();
    // let mut flash = Flash::new();
    // // flash.write(0x0001_8314, &[0x1111, 0x2222, 0x3333, 0x4444, 0x0000]);
    // let mut buffer: [u16; 2] = [0x0000, 0x0000];
    // flash.read(0x0001_830a, &mut buffer);

    // if buffer[2] == 0x3323 {
    //     pc13.set_low();
    // } else {
    //     pc13.set_high();
    // }

    // let mut fprom = Fprom::new(0x0000_f800);
    // fprom.write(&[1023, 1024, 1025]);

    // for i in 0..1024 {
    //     fprom.write(&[i]);
    // }
    
    let flash = Flash::new();
    flash.set_latency(2);
    flash.keyr.write(0x4567_0123);
    flash.keyr.write(0xcdef_89ab);

    if flash.get_cr_lock() == 1 {
        flash.unlock_cr();
    }
    while flash.get_cr_lock() == 1 {};

    while flash.sr.get_bit(0) == 1 {};
    if flash.sr.get_bit(5) == 1 {
        flash.sr.write(0x0000_0020);
    }

    flash.cr.set_bit(1);
    while flash.sr.get_bit(0) == 1 {};
    flash.ar.write(0x0800_f800);
    while flash.sr.get_bit(0) == 1 {};
    flash.cr.set_bit(6);
    while flash.sr.get_bit(5) == 0 {};
    flash.sr.write(0x0000_0020);
    flash.cr.reset_bit(1);
    
    // pc13.set_low();

    // fprom.flash.erase_page(0x0001_8300, &pc13);

    // pc13.set_low();

    // fprom.read(&mut buffer);

    // if buffer[1] == 0x1111 {

    //     pc13.set_low();
    // }
    // else {
    //     pc13.set_high();
    // }
    loop {}
}
