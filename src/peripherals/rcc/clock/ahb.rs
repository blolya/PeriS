use super::{ Clock, super::Rcc, sys::SystemClock };

pub struct Ahb {
    rcc: Rcc,
}
impl Ahb {
    pub fn new() -> Ahb {
        Ahb {
            rcc: Rcc::new(),
        }
    }
}
impl Clock for Ahb {
    fn get_input_frequency(&self) -> u32 {
        let bin_prescaler = self.rcc.get_ahb_prescaler();
        let prescaler = match bin_prescaler {
            8 => AhbPrescaler::Db2,
            9 => AhbPrescaler::Db4,
            10 => AhbPrescaler::Db8,
            11 => AhbPrescaler::Db16,
            12 => AhbPrescaler::Db64,
            13 => AhbPrescaler::Db128,
            14 => AhbPrescaler::Db256,
            15 => AhbPrescaler::Db512,
            _ => panic!("AhbPrescaler unknown value"),
        };
        SystemClock::new().get_output_frequency() / prescaler as u32
    }
    fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}
pub enum AhbPrescaler {
    Db2 = 2,
    Db4 = 4,
    Db8 = 8,
    Db16 = 16,
    Db64 = 64,
    Db128 = 128,
    Db256 = 256,
    Db512 = 512,
}
impl From<u32> for AhbPrescaler {
    fn from(prescaler: u32) -> AhbPrescaler {
        match prescaler {
            2 => AhbPrescaler::Db2,
            4 => AhbPrescaler::Db4,
            8 => AhbPrescaler::Db8,
            16 => AhbPrescaler::Db16,
            64 => AhbPrescaler::Db64,
            128 => AhbPrescaler::Db128,
            256 => AhbPrescaler::Db256,
            512 => AhbPrescaler::Db512,
            _ => panic!(""),
        }
    }
}
