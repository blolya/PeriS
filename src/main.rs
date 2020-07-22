#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::rcc::{ Rcc, SystemClock };
use peris::core::Register;

#[entry]
fn main() -> ! {

    let system_clock = SystemClock::new();
    let system_clock_source = system_clock.get_source();


    // let rcc = Rcc::new();
    // rcc.cr.set_bit(16);
    // let mut hse_status = rcc.cr.get_bit(17);
    // while hse_status == 0 {
    //     hse_status = rcc.cr.get_bit(17);
    // };
    // rcc.cfgr.write_or(0x001C_0000); 
    // rcc.cfgr.set_bit(16);
    // rcc.cr.set_bit(24);
    // let mut pll_status = rcc.cr.get_bit(25);
    // while pll_status == 0 {
    //     pll_status = rcc.cr.get_bit(25);
    // };

    // let flash_register = Register::new(0x4002_2000);
    // flash_register.write_or(0x0000_0002);

    // rcc.cfgr.write_or(0x0000_0400);
    // rcc.cfgr.write_or(0x0000_0002);

    // let mut pll_clock_status = rcc.cfgr.read() & 0x0000_0003;
    // while pll_clock_status != 2 {
    //     pll_clock_status = rcc.cfgr.read() & 0x0000_0003;
    // };


    loop {}
}
