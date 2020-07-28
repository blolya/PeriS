use super::{ hsi::Hsi, hse::Hse, Clock, super::{ Rcc, super::Device } };

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
        let bin_source: u32 = source.into();
        self.rcc.set_pll_clock_source(bin_source);

        let mut clock_source: u32 = self.get_source().into();
        let mut cycles = 0;
        while clock_source != bin_source {
            clock_source = self.get_source().into();

            cycles += 1;
            if cycles > 100 {
                panic!("Unable to switch system clock source");
            }
        }
    }
    pub fn get_source(&self) -> PllClockSource {
        let source: PllClockSource = self.rcc.get_pll_clock_source().into();
        source
    }
    pub fn set_multiplication_factor(&self, factor: PllMul) {
        self.rcc.set_pll_multiplication_factor(factor as u32);
    }
    pub fn get_pll_multiplication_factor(&self) -> PllMul {
        let bin_factor = self.rcc.get_pll_multiplication_factor();
        let factor = match bin_factor {
            0 => PllMul::Pllx2,
            1 => PllMul::Pllx3,
            2 => PllMul::Pllx4,
            3 => PllMul::Pllx5,
            4 => PllMul::Pllx6,
            5 => PllMul::Pllx7,
            6 => PllMul::Pllx8,
            7 => PllMul::Pllx9,
            8 => PllMul::Pllx10,
            9 => PllMul::Pllx11,
            10 => PllMul::Pllx12,
            11 => PllMul::Pllx13,
            12 => PllMul::Pllx14,
            13 => PllMul::Pllx15,
            14 => PllMul::Pllx16,
            15 => PllMul::Pllx16,
            _ => panic!("Unknown pll multiplication factor"),
        };
        factor
    }
    pub fn set_prescaler(&self, prescaler: PllPrescaler) {
        let bin_prescaler = match prescaler {
            PllPrescaler::Db1 => 0,
            PllPrescaler::Db2 => 1,
            _ => panic!("Unknown pll prescaler"),
        };
        self.rcc.set_pll_prescaler(bin_prescaler);
    }
    pub fn get_prescaler(&self) -> PllPrescaler {
        let bin_prescaler = self.rcc.get_pll_prescaler();
        match bin_prescaler {
            0 => PllPrescaler::Db1,
            1 => PllPrescaler::Db2,
            _ => panic!("Unknown pll prescaler"),
        }
    }
}
impl Clock for Pll {
    fn get_input_frequency(&self) -> u32 {
        let clock_source = self.get_source();
        match clock_source {
            PllClockSource::Hsi(source) => {
                source.get_output_frequency()
            },
            PllClockSource::Hse(source) => {
                source.get_output_frequency() / self.get_prescaler() as u32
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
    Pllx2 = 2,
    Pllx3 = 3,
    Pllx4 = 4,
    Pllx5 = 5,
    Pllx6 = 6,
    Pllx7 = 7,
    Pllx8 = 8,
    Pllx9 = 9,
    Pllx10 = 10,
    Pllx11 = 11,
    Pllx12 = 12,
    Pllx13 = 13,
    Pllx14 = 14,
    Pllx15 = 15,
    Pllx16 = 16,
}
impl From<u32> for PllMul {
    fn from(mul: u32) -> PllMul {
        match mul {
            2 => PllMul::Pllx2,
            3 => PllMul::Pllx3,
            4 => PllMul::Pllx4,
            5 => PllMul::Pllx5,
            6 => PllMul::Pllx6,
            7 => PllMul::Pllx7,
            8 => PllMul::Pllx8,
            9 => PllMul::Pllx9,
            10 => PllMul::Pllx10,
            11 => PllMul::Pllx11,
            12 => PllMul::Pllx12,
            13 => PllMul::Pllx13,
            14 => PllMul::Pllx14,
            15 => PllMul::Pllx15,
            16 => PllMul::Pllx16,
            _ => panic!(""),
        }
    }
}
pub enum PllPrescaler {
    Db1 = 1,
    Db2 = 2,
}
impl From<u32> for PllPrescaler {
    fn from(prescaler: u32) -> PllPrescaler {
        match prescaler {
            1 => PllPrescaler::Db1,
            2 => PllPrescaler::Db2,
            _ => panic!(""),
        }
    }
}

pub enum PllClockSource {
    Hsi(Hsi),
    Hse(Hse),
}
impl From<u32> for PllClockSource {
    fn from(source: u32) -> PllClockSource {
        match source {
            0 => PllClockSource::Hsi(Hsi::new()),
            1 => PllClockSource::Hse(Hse::new()),
            _ => panic!("There is no pll clock source with bin code = {}", source),
        }
    }
}
impl From<PllClockSource> for u32 {
    fn from(source: PllClockSource) -> u32 {
        match source {
            PllClockSource::Hsi(_) => 0,
            PllClockSource::Hse(_) => 1,
        }
    }
} 