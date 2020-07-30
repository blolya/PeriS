use super::{ ahb::Ahb, super::Rcc };

pub struct Apb2 {
    rcc: Rcc,
}
impl Apb2 {
    pub fn new() -> Apb2 {
        Apb2 {
            rcc: Rcc::new(),
        }
    }
    pub fn set_prescaler(&self, prescaler: Apb2Prescaler) {
        let bin_prescaler = match prescaler {
            Apb2Prescaler::Db1 => 0,
            Apb2Prescaler::Db2 => 4,
            Apb2Prescaler::Db4 => 5,
            Apb2Prescaler::Db8 => 6,
            Apb2Prescaler::Db16 => 7,
        };
        self.rcc.set_apb1_prescaler(bin_prescaler)
    }
    pub fn get_prescaler(&self) -> Apb2Prescaler {
        match self.rcc.get_apb2_prescaler() {
            0..=3 => Apb2Prescaler::Db1,
            4 => Apb2Prescaler::Db2,
            5 => Apb2Prescaler::Db4,
            6 => Apb2Prescaler::Db8,
            7 => Apb2Prescaler::Db16,
            _ => panic!("Apb2Prescaler"),
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        let bin_prescaler = self.rcc.get_apb2_prescaler();
        let prescaler = match bin_prescaler {
            0..=3 => Apb2Prescaler::Db1,
            4 => Apb2Prescaler::Db2,
            5 => Apb2Prescaler::Db4,
            6 => Apb2Prescaler::Db8,
            7 => Apb2Prescaler::Db16,
            _ => panic!("Apb2Prescaler unknown value"),
        };
        Ahb::new().get_output_frequency() / prescaler as u32
    }
    pub fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}
pub enum Apb2Prescaler {
    Db1 = 1,
    Db2 = 2,
    Db4 = 4,
    Db8 = 8,
    Db16 = 16,
}
impl From<u32> for Apb2Prescaler {
    fn from(prescaler: u32) -> Apb2Prescaler {
        match prescaler {
            1 => Apb2Prescaler::Db1,
            2 => Apb2Prescaler::Db2,
            4 => Apb2Prescaler::Db4,
            8 => Apb2Prescaler::Db8,
            16 => Apb2Prescaler::Db16,
            _ => panic!(""),
        }
    }
}
