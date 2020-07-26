use super::super::{ super::Device, Rcc, super::super::core::Register };
use super::{ hsi::Hsi, hse::Hse, pll::Pll };
use super::Clock;


pub struct SystemClock {
    rcc: Rcc,
}
impl SystemClock {
    pub fn new() -> SystemClock {
        SystemClock {
            rcc: Rcc::new(),
        }
    }
    pub fn set_source(&self, source: SystemClockSource) {
        match source {
            SystemClockSource::Hsi(clock_source) => {
                panic!("Hsi not enabled");
            },
            SystemClockSource::Hse(clock_source) => {
                clock_source.enable();
            },
            SystemClockSource::Pll(clock_source) => {
                clock_source.enable();
            },
        }
        let bin_source: u32 = source.into();
        let flash_register = Register::new(0x4002_2000);
        flash_register.write_or(0x0000_0002);

        self.rcc.set_system_clock_source(bin_source);

        let mut clock_source: u32 = self.get_source().into();
        let mut cycles = 0;
        while clock_source != bin_source {
            clock_source = self.rcc.get_system_clock_source();

            cycles += 1;
            if cycles > 100 {
                panic!("Unable to switch system clock source");
            }
        }
    }
    pub fn get_source(&self) -> SystemClockSource {
        match self.rcc.get_system_clock_source() {
            0 => SystemClockSource::Hsi(Hsi::new()),
            1 => SystemClockSource::Hse(Hse::new()),
            2 => SystemClockSource::Pll(Pll::new()),
            _ => panic!("Clock not defined"),
        }
    }
}
impl Clock for SystemClock {
    fn get_input_frequency(&self) -> u32 {
        let source_type = self.get_source();
        let input_frequency = if let SystemClockSource::Hse(clock_source) = source_type {
            clock_source.get_output_frequency()
        } else if let SystemClockSource::Pll(clock_source) = source_type {
            clock_source.get_output_frequency()
        } else {
            12
        };
        input_frequency
    }
    fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}

pub enum SystemClockSource {
    Hsi(Hsi),
    Hse(Hse),
    Pll(Pll),
}
impl From<u32> for SystemClockSource {
    fn from(source: u32) -> SystemClockSource {
        match source {
            0 => SystemClockSource::Hsi(Hsi::new()),
            1 => SystemClockSource::Hse(Hse::new()),
            2 => SystemClockSource::Pll(Pll::new()),
            _ => panic!("There is no clock source with bin code = {}", source),
        }
    }
}
impl From<SystemClockSource> for u32 {
    fn from(source: SystemClockSource) -> u32 {
        match source {
            SystemClockSource::Hsi(_) => 0,
            SystemClockSource::Hse(_) => 1,
            SystemClockSource::Pll(_) => 2,
        }
    }
}
