#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    communication::usb::Usb,
    communication::uart::usart1::Usart1,
    clock,
    ports::{
        Port,
        PortMode,
        PortNum,
        OutputConfig,
        MaxSpeed,
    },
};
use peris::core::gpio::gpioc::Gpioc;

#[entry]
fn main() -> ! {
    clock::init();
    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz)), &gpioc);
    pc13.set_high();
    let dbgr = Usart1::new();
    let usb = Usb::new();
    usb.reset();

    loop {
        if usb.get_correct_transfer_status() == 1 {
            usb.correct_transfer();
        }
        if usb.get_pma_over_status() == 1 {
            usb.pma_over();
        }
        if usb.get_err_status() == 1 {
            usb.err();
        }
        if usb.get_wake_up_status() == 1 {
            usb.wake_up();
        }
        if usb.get_suspend_mode_status() == 1 {
            usb.suspend_mode();
        }
        if usb.get_reset_status() == 1 {
            usb.reset();
        }
        if usb.get_start_of_frame_status() == 1 {
            usb.start_of_frame();
        }
        if usb.get_expected_start_of_frame_status() == 1 {
            usb.expected_start_of_frame();
        }
        
    }
}
