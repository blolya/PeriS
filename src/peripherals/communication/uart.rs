pub mod usart1;

use super::CommunicationInterface;
pub trait Uart: CommunicationInterface {
    fn enable(&self);
    fn disable(&self);
    fn set_word_length(&self, word_length: u32);
    fn set_stop_bits_num(&self, stop_bits_num: u32);
    fn set_baud_rate(&self, baud_rate: u32);
    fn enable_transmitter(&self);
    fn send(&self, data: u32);
}