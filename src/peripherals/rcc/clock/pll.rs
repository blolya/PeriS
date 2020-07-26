use super::{ hse::Hse, Clock, super::{ Rcc, super::Device } };


pub struct Pll {
    rcc: Rcc,
}
impl Pll {
    pub fn new() -> Pll {
        Pll {
            rcc: Rcc::new(),
        }
    }
    pub fn set_source(&self, source: PllClockSource) {
        let bin_source: u32 = (&source).into();

        match source {
            PllClockSource::Hsi => panic!("Hsi not implemented"),
            PllClockSource::Hse(clock_source) => {
                clock_source.enable();
            },
        }
        self.rcc.set_pll_clock_source(bin_source);
    }
    pub fn get_source(&self) -> PllClockSource {
        let source: PllClockSource = self.rcc.get_pll_clock_source().into();
        source
    }
    pub fn set_multiplication_factor(&self, factor: PllMul) {
        self.rcc.set_pll_multiplication_factor(factor as u32);
    }
    pub fn get_pll_multiplication_factor(&self) -> PllMul {
        let factor: PllMul = self.rcc.get_pll_multiplication_factor().into();
        factor
    }
}
impl Clock for Pll {
    fn get_input_frequency(&self) -> u32 {
        let clock_source = self.get_source();
        match clock_source {
            PllClockSource::Hsi => panic!("Hsi not implemented"),
            PllClockSource::Hse(source) => {
                source.get_output_frequency()
            },
        }
    }
    fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency() * self.get_pll_multiplication_factor() as u32
    }
}
impl Device for Pll {
    fn enable(&self) {
        self.rcc.enable_pll();
        let mut pll_ready_status = self.rcc.get_pll_ready_status();

        let mut cycles = 0;
        while pll_ready_status == 0 {
            pll_ready_status = self.rcc.get_pll_ready_status();

            cycles += 1;
            if cycles > 100 {
                panic!("Unable to enable Pll");
            }
        }
    } 
    fn disable(&self) {
        self.rcc.disable_pll();
        let mut pll_ready_status = self.rcc.get_pll_ready_status();

        let mut cycles = 0;
        while pll_ready_status == 1 {
            pll_ready_status = self.rcc.get_pll_ready_status();

            cycles += 1;
            if cycles > 100 {
                panic!("Unable to disable Pll");
            }
        }
    }
}
pub enum PllMul {
    Pllx9 = 7,
}
impl From<u32> for PllMul {
    fn from(mul: u32) -> PllMul {
        match mul {
            7 => PllMul::Pllx9,
            _ => panic!(""),
        }
    }
}

pub enum PllClockSource {
    Hsi,
    Hse(Hse),
}
impl From<u32> for PllClockSource {
    fn from(source: u32) -> PllClockSource {
        match source {
            0 => PllClockSource::Hsi,
            1 => PllClockSource::Hse(Hse::new()),
            _ => panic!("There is no pll clock source with bin code = {}", source),
        }
    }
}
impl From<&PllClockSource> for u32 {
    fn from(source: &PllClockSource) -> u32 {
        match source {
            PllClockSource::Hsi => 0,
            PllClockSource::Hse(_) => 1,
        }
    }
} 