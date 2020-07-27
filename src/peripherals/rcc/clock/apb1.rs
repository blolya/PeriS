pub enum Apb1Prescaler {
    Db2 = 4,
}
impl From<u32> for Apb1Prescaler {
    fn from(prescaler: u32) -> Apb1Prescaler {
        match prescaler {
            4 => Apb1Prescaler::Db2,
            _ => panic!(""),
        }
    }
}
// pub fn set_apb1_prescaler(&self, prescaler: Apb1Prescaler) {
//     self.rcc.set_apb1_prescaler(prescaler as u32);
// }
// pub fn get_apb1_prescaler(&self) -> Apb1Prescaler {
//     let prescaler: Apb1Prescaler = self.rcc.get_apb1_prescaler().into();
//     prescaler
// }