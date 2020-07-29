use super::{ 
    hsi::Hsi, hse::Hse, 
    super::Rcc 
};

pub struct Pll {
    rcc: Rcc,
}
impl Pll {
    pub fn new() -> Pll {
        Pll {
            rcc: Rcc::new(),
        }
    }
    pub fn enable(&self) {
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
    pub fn disable(&self) {
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
    pub fn set_clock_source(&self, clock_source: PllClockSource) {
        match clock_source {
            PllClockSource::Hsi => self.rcc.set_pll_clock_source(0),
            PllClockSource::Hse => self.rcc.set_pll_clock_source(1),
        }

        let mut new_clock_source = self.get_clock_source();
        let mut cycles = 0;
        while new_clock_source != clock_source {
            new_clock_source = self.get_clock_source();

            cycles += 1;
            if cycles > 100 {
                panic!("Unable to switch system clock source");
            }
        }
    }
    pub fn get_clock_source(&self) -> PllClockSource {
        match self.rcc.get_pll_clock_source() {
            0 => PllClockSource::Hsi,
            1 => PllClockSource::Hse,
            _ => panic!(""),
        }
    }
    pub fn set_mul(&self, factor: PllMul) {
        let bin_factor = match factor {
            PllMul::Pllx2 => 0,
            PllMul::Pllx3 => 1,
            PllMul::Pllx4 => 2,
            PllMul::Pllx5 => 3,
            PllMul::Pllx6 => 4,
            PllMul::Pllx7 => 5,
            PllMul::Pllx8 => 6,
            PllMul::Pllx9 => 7,
            PllMul::Pllx10 => 8,
            PllMul::Pllx11 => 9,
            PllMul::Pllx12 => 10,
            PllMul::Pllx13 => 11,
            PllMul::Pllx14 => 12,
            PllMul::Pllx15 => 13,
            PllMul::Pllx16 => 14,
        };
        self.rcc.set_pll_multiplication_factor(bin_factor);
    }
    pub fn get_mul(&self) -> PllMul {
        match self.rcc.get_pll_multiplication_factor() {
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
        }
    }
    pub fn set_hse_prescaler(&self, prescaler: PllHsePrescaler) {
        let bin_prescaler = match prescaler {
            PllHsePrescaler::Db1 => 0,
            PllHsePrescaler::Db2 => 1,
        };
        self.rcc.set_pll_prescaler(bin_prescaler);
    }
    pub fn get_hse_prescaler(&self) -> PllHsePrescaler {
        let bin_prescaler = self.rcc.get_pll_prescaler();
        match bin_prescaler {
            0 => PllHsePrescaler::Db1,
            1 => PllHsePrescaler::Db2,
            _ => panic!("Unknown pll prescaler"),
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        let clock_source = self.get_clock_source();
        match clock_source {
            PllClockSource::Hsi => Hsi::new().get_output_frequency(),
            PllClockSource::Hse => Hse::new().get_output_frequency() / self.get_hse_prescaler() as u32,
        }
    }
    pub fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency() * self.get_mul() as u32
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

#[derive(PartialEq)]
pub enum PllHsePrescaler {
    Db1 = 1,
    Db2 = 2,
}
#[derive(PartialEq)]
pub enum PllClockSource {
    Hsi,
    Hse,
}