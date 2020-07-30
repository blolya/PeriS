pub mod gpio;
pub mod rcc;
pub mod flash;
pub mod communication;

pub trait Device {
    fn enable(&self);
    fn disable(&self);
}
pub trait Clocked {
    fn enable_clock();
    fn disable_clock();
}
