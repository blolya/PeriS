pub mod hsi;
pub mod hse;
pub mod pll;
pub mod sys;
pub mod ahb;
pub mod apb1;
pub mod apb2;

pub trait Clock: Sized {
    fn get_input_frequency(&self) -> u32;
    fn get_output_frequency(&self) -> u32;
}