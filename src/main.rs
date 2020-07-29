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

    // let hse = Hse::new();
    // let pll = Pll::new();
    // pll.set_multiplication_factor(PllMul::Pllx9);
    // pll.set_source(PllClockSource::Hse( hse ));
    // pll.enable();

    // let system_clock = SystemClock::new();
    // let apb1 = Apb1::new();
    // apb1.set_prescaler(Apb1Prescaler::Db2);
    // system_clock.set_source(SystemClockSource::Pll( pll ));

    let ua1 = Usart1::new();
    let arr: () = "Ku\r".as_bytes().iter().map( |c| {
        ua1.send(*c);
    }).collect();

    loop {}
}
