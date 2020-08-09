use super::{
    sys::SystemClock, 
    super::super::core::rcc::Rcc,
};

pub struct Ahb {
    rcc: Rcc,
}
impl Ahb {
    pub fn new() -> Ahb {
        Ahb {
            rcc: Rcc::new(),
        }
    }
    pub fn set_prescaler(&self, prescaler: AhbPrescaler) {
        let bin_prescaler = match prescaler {
            AhbPrescaler::Db1 => 0,
            AhbPrescaler::Db2 => 8,
            AhbPrescaler::Db4 => 9,
            AhbPrescaler::Db8 => 10,
            AhbPrescaler::Db16 => 11,
            AhbPrescaler::Db64 => 12,
            AhbPrescaler::Db128 => 13, 
            AhbPrescaler::Db256 => 14,
            AhbPrescaler::Db512 => 15,
        };
        self.rcc.set_ahb_prescaler(bin_prescaler)
    }
    pub fn get_prescaler(&self) -> AhbPrescaler {
        match self.rcc.get_ahb_prescaler() {
            0..=7 => AhbPrescaler::Db1,
            8  => AhbPrescaler::Db2,
            9  => AhbPrescaler::Db4,
            10 => AhbPrescaler::Db8,
            11 => AhbPrescaler::Db16,
            12 => AhbPrescaler::Db64,
            13 => AhbPrescaler::Db128,
            14 => AhbPrescaler::Db256,
            15 => AhbPrescaler::Db512,
            _ => panic!("AhbPrescaler"),
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        let bin_prescaler = self.rcc.get_ahb_prescaler();
        let prescaler = match bin_prescaler {
            0..=7 => AhbPrescaler::Db1,
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
    pub fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}
pub enum AhbPrescaler {
    Db1 = 1,
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
            1 => AhbPrescaler::Db1,
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
