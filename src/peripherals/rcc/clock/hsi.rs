use super::{ Clock, super::{ Rcc, super::Device } };

pub struct Hsi {
    rcc: Rcc,
    input_frequency: u32,
}
impl Hsi {
    pub fn new() -> Hsi {
        Hsi {
            rcc: Rcc::new(),
            input_frequency: 8,
        }
    }
}
impl Clock for Hsi {
    fn get_input_frequency(&self) -> u32 {
        self.input_frequency
    }
    fn get_output_frequency(&self) -> u32 {
        self.input_frequency
    }
}
impl Device for Hsi {
    fn enable(&self) {
        self.rcc.enable_hsi();
        let mut hsi_ready_status = self.rcc.get_hsi_ready_status();

        let mut cycles = 0;
        while hsi_ready_status == 0 {
            hsi_ready_status = self.rcc.get_hsi_ready_status();
    
            cycles += 1;
            if cycles > 100 {
                panic!("Can't enable Hsi");
            }
        }
    } 
    fn disable(&self) {
        self.rcc.disable_hsi();
        let mut hsi_ready_status = self.rcc.get_hsi_ready_status();

        let mut cycles = 0;
        while hsi_ready_status == 1 {
            hsi_ready_status = self.rcc.get_hsi_ready_status();

            cycles += 1;
            if cycles > 100 {
                panic!("Can't disable Hsi");
            }
        }
    }
}