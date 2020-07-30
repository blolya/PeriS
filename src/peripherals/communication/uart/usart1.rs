use super::{ 
    Uart, 
    CommunicationInterface, 
    super:: { 
        Clocked, 
        super::rcc::Rcc,
        super::super::core::Register,
        super::gpio::{ 
            gpioa::Gpioa,
            port::{
                Port,
                PortMode,
                MaxSpeed,
                PortNum,
                OutputConfig,
            },
        }
    },
};
use super::super::super::rcc::clock::apb2::Apb2;
pub struct Usart1 {
    sr: Register,
    dr: Register,
    brr: Register,
    cr1: Register,
    cr2: Register,
    cr3: Register,
    gtpr: Register,
}
impl Usart1 {
    pub fn new() -> Usart1 {
        Rcc::new().enable_afio();
        Usart1::enable_clock();
        let address = 0x4001_3800;
        let usart = Usart1 {
            sr: Register::new(address),
            dr: Register::new(address + 0x04),
            brr: Register::new(address + 0x08),
            cr1: Register::new(address + 0x0C),
            cr2: Register::new(address + 0x10),
            cr3: Register::new(address + 0x14),
            gtpr: Register::new(address + 0x18),
        };
        usart.enable();
        usart.set_word_length(0);
        usart.set_stop_bits_num(0);
        usart.set_baud_rate(9600);
        usart.enable_transmitter();

        usart
    }
}
impl Uart for Usart1 {
    fn enable(&self) {
        self.cr1.set_bit( 13 );
    }
    fn disable(&self) {
        self.cr1.reset_bit( 13 );
    }
    fn set_word_length(&self, word_length: u32) {
        self.cr1.write_bit(12, word_length);
    }
    fn set_stop_bits_num(&self, stop_bits_num: u32) {
        self.cr2.write_and(0x0000_0000);
        self.cr2.write_and(!(0b11 << 12));
        self.cr2.write_or(stop_bits_num << 12);
    }
    fn set_baud_rate(&self, baud_rate: u32) {
        let apb2_frequency = Apb2::new().get_output_frequency();
        let baud_rate = (apb2_frequency * 1_000_000) as f32 / (baud_rate as f32 * 16.0) - 1.0;
        let bin_baud_rate = (baud_rate as u32) << 4 | (baud_rate % 1.0 * 16.0) as u32;

        self.brr.write(bin_baud_rate);
    }
    fn enable_transmitter(&self) {
        let gpioa = Gpioa::new();
        let port_mode = PortMode::Output( OutputConfig::AlternativeFunctionPushPull( MaxSpeed::S2MHz ) );
        Port::new(PortNum::P9, port_mode, &gpioa);

        self.cr1.set_bit(3);
    }
    fn send(&self, data: u32) {
        let mut status = self.sr.get_bit(7);
        while status != 1 {
            status = self.sr.get_bit(7);
        }
        self.dr.write(data);
    }
}
impl CommunicationInterface for Usart1 {

}
impl Clocked for Usart1 {
    fn enable_clock() {
        Rcc::new().enable_usart1();
    }
    fn disable_clock() {
        Rcc::new().disable_usart1();
    }
}