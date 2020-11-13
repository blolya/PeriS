use super::{
    pll::Pll,
    super::super::core::rcc::Rcc,
};

pub struct Usb {
    rcc: Rcc,
}
impl Usb {
    pub fn new() -> Usb {
        Usb {
            rcc: Rcc::new(),
        }
    }
    pub fn set_prescaler(&self, prescaler: UsbPrescaler) {
        let bin_prescaler = match prescaler {
            UsbPrescaler::Db1 => 1,
            UsbPrescaler::Db1_5 => 0,
        };
        self.rcc.set_usb_prescaler(bin_prescaler);
    }
    pub fn get_prescaler(&self) -> UsbPrescaler {
        match self.rcc.get_usb_prescaler() {
            0 => UsbPrescaler::Db1_5,
            1 => UsbPrescaler::Db1,
            _ => panic!("UsbPrescaler"),
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        let prescaler = self.get_prescaler();
        Pll::new().get_output_frequency() / prescaler as u32
    }
    pub fn get_output_frequency(&self) -> u32 {
        self.get_input_frequency()
    }
}
pub enum UsbPrescaler {
    Db1,
    Db1_5,
}
impl From<f32> for UsbPrescaler {
    fn from(prescaler: f32) -> UsbPrescaler {
        match prescaler {
            1.0 => UsbPrescaler::Db1,
            1.5 => UsbPrescaler::Db1_5,
            _ => panic!(""),
        }
    }
}
impl From<UsbPrescaler> for f32 {
    fn from(prescaler: UsbPrescaler) -> f32 {
        match prescaler {
            UsbPrescaler::Db1 => 1.0,
            UsbPrescaler::Db1_5 => 1.5,
        }
    }
}
