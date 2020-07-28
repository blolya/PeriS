#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;


use peris::peripherals::rcc::clock::{ sys::{SystemClock, SystemClockSource}, pll::{ Pll, PllMul, PllClockSource }, hse::Hse, apb1:: { Apb1, Apb1Prescaler} };
use peris::peripherals::Device;
use peris::peripherals::communication::{uart::Uart, uart::usart1::Usart1};

#[entry]
fn main() -> ! {

    let hse = Hse::new();
    let pll = Pll::new();
    pll.set_multiplication_factor(PllMul::Pllx9);
    pll.set_source(PllClockSource::Hse( hse ));
    pll.enable();

    let system_clock = SystemClock::new();
    let apb1 = Apb1::new();
    apb1.set_prescaler(Apb1Prescaler::Db2);
    system_clock.set_source(SystemClockSource::Pll( pll ));

    let ua1 = Usart1::new();
    let arr: () = "Ku\r".as_bytes().iter().map( |c| {
        ua1.send(*c);
    }).collect();

    loop {}
}
