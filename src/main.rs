#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
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

    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output( OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz) ), &gpioc);

    let flash = Flash::new();

    if flash.get_cr_lock() == 1 {
        flash.unlock_cr();
    }
    while flash.get_cr_lock() == 1 {};

    while flash.get_sr_bsy() == 1 {};
    flash.reset_eop();

    flash.select_programming();
    unsafe {
        let addr = 0x0801_8300 as *mut u16;
        *addr = 0x9abc; 
    }

    while flash.get_eop() == 0 {};
    flash.reset_eop();
    unsafe {
        let addr = 0x0801_8302 as *mut u16;
        *addr = 0xdef0; 
    }

    while flash.get_eop() == 0 {};
    flash.reset_eop();

    flash.unselect_programming();

    pc13.set_low();


    loop {}
}
