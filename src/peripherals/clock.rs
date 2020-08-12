pub mod hsi;
pub mod hse;
pub mod pll;
pub mod sys;
pub mod ahb;
pub mod apb1;
pub mod apb2;

use self::{
    hse::Hse,
    pll::{
        Pll,
        PllMul,
        PllClockSource,
        PllHsePrescaler
    },
    sys::{
        SystemClock,
        SystemClockSource
    },
    apb1::{
        Apb1,
        Apb1Prescaler
    },
};

pub fn init() {
    let hse = Hse::new();
    hse.enable();

    let pll = Pll::new();
    pll.set_mul(PllMul::Pllx9);
    pll.set_hse_prescaler(PllHsePrescaler::Db1);
    pll.set_clock_source(PllClockSource::Hse);
    pll.enable();

    let apb1 = Apb1::new();
    apb1.set_prescaler(Apb1Prescaler::Db2);

    let sys = SystemClock::new();
    sys.set_clock_source(SystemClockSource::Pll);
}