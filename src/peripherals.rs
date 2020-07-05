pub mod gpio;
pub mod rcc;

pub trait Clocked {
    fn enable_clock();
    fn disable_clock();
}
