#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    communication::uart::{
        Uart, usart1::Usart1
    },
    rcc::clock::{ 
        hse::Hse, 
        pll::{ Pll, PllMul, PllClockSource },
        sys::{ SystemClock, SystemClockSource },
        apb1:: { Apb1, Apb1Prescaler },
    }
};
use peris::peripherals::rcc::clock::apb2::Apb2;

#[entry]
fn main() -> ! {
    let hse = Hse::new();
    hse.enable();

    let pll = Pll::new();
    pll.set_mul(PllMul::Pllx9);
    pll.set_clock_source(PllClockSource::Hse);
    pll.enable();

    let apb1 = Apb1::new();
    apb1.set_prescaler(Apb1Prescaler::Db2);

    let sys = SystemClock::new();
    sys.set_clock_source(SystemClockSource::Pll);


    let ua1 = Usart1::new();
    ua1.send('L' as u32);
    ua1.send('a' as u32);
    ua1.send('\r' as u32);

    loop {}
}
