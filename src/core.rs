use core::ptr;

pub struct Register {
    address: u32,
}
impl Register {
    pub fn new(address: u32) -> Register {
        Register { address }
    }
    pub fn write(&self, value: u32) {
        unsafe { ptr::write_volatile(self.address as *mut u32, value) }
    }
    pub fn write_and(&self, value: u32) {
        let old_value = self.read();
        self.write(value & old_value);
    }
    pub fn write_or(&self, value: u32) {
        let old_value = self.read();
        self.write(value | old_value);
    }
    pub fn read(&self) -> u32 {
        let value;
        unsafe {
            value = ptr::read_volatile(self.address as *mut u32);
        }
        value
    }
}
