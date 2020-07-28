use super::{ Clock, ahb::Ahb, super::Rcc };

pub struct Apb1 {
    rcc: Rcc,
}
impl Apb1 {
    pub fn new() -> Apb1 {
        Apb1 {
            rcc: Rcc::new(),
        }
    }
    pub fn set_prescaler(&self, prescaler: Apb1Prescaler) {
        let bin_prescaler = match prescaler {
            Apb1Prescaler::Db2 => 4,
            Apb1Prescaler::Db4 => 5,
            Apb1Prescaler::Db8 => 6,
            Apb1Prescaler::Db16 => 7,
        };
        self.rcc.set_apb1_prescaler(bin_prescaler)
    }
}
impl Clock for Apb1 {
    fn get_input_frequency(&self) -> u32 {
        let bin_prescaler = self.rcc.get_apb1_prescaler();
        let prescaler = match bin_prescaler {
            4 => Apb1Prescaler::Db2,
            5 => Apb1Prescaler::Db4,
            6 => Apb1Prescaler::Db8,
            7 => Apb1Prescaler::Db16,
            _ => panic!("Apb1Prescaler unknown value"),
        };
        Ahb::new().get_output_frequency() / prescaler as u32
    }
    fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}
pub enum Apb1Prescaler {
    Db2 = 2,
    Db4 = 4,
    Db8 = 8,
    Db16 = 16,
}
impl From<u32> for Apb1Prescaler {
    fn from(prescaler: u32) -> Apb1Prescaler {
        match prescaler {
            2 => Apb1Prescaler::Db2,
            4 => Apb1Prescaler::Db4,
            8 => Apb1Prescaler::Db8,
            16 => Apb1Prescaler::Db16,
            _ => panic!(""),
        }
    }
}
