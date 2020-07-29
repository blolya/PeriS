use super:: {
    hsi::Hsi, hse::Hse, pll::Pll,
    super:: Rcc,
};


pub struct SystemClock {
    rcc: Rcc,
}
impl SystemClock {
    pub fn new() -> SystemClock {
        SystemClock {
            rcc: Rcc::new(),
        }
    }
    pub fn set_clock_source(&self, clock_source: SystemClockSource) {
        match clock_source {
            SystemClockSource::Hsi => self.rcc.set_system_clock_source(0),
            SystemClockSource::Hse => self.rcc.set_system_clock_source(1),
            SystemClockSource::Pll => self.rcc.set_system_clock_source(2),
        };

        let mut new_clock_source = self.get_clock_source();
        while new_clock_source != clock_source {
            new_clock_source = self.get_clock_source();
        }
    }
    pub fn get_clock_source(&self) -> SystemClockSource {
        let bin_clock_source = self.rcc.get_system_clock_source();
        match bin_clock_source {
            0 => SystemClockSource::Hsi,
            1 => SystemClockSource::Hse,
            2 => SystemClockSource::Pll,
            _ => panic!(""),
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        let bin_clock_source = self.rcc.get_system_clock_source();
        match bin_clock_source {
            0 => Hsi::new().get_output_frequency(),
            1 => Hse::new().get_output_frequency(),
            2 => Pll::new().get_output_frequency(),
            _ => panic!(""),
        }
    }
    pub fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}

#[derive(PartialEq)]
pub enum SystemClockSource {
    Hsi,
    Hse,
    Pll,
}
