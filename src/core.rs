pub struct Register {
    address: *mut u32,
}
impl Register {
    pub fn new(address: u32) -> Register {
        Register {
            address: address as *mut u32,
        }
    }
    pub fn write(&self, value: u32) {
        unsafe {
            *self.address = value;
        }
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
            value = *self.address;
        }
        value
    }
    pub fn write_bit(&self, bit: u32, value: u32) {
        match value {
            0 => self.write_and(!(0b1 << bit)),
            1 => self.write_or(0b1 << bit),
            _ => panic!("Bit value can be 0 or 1"),
        }
    }
    pub fn set_bit(&self, bit: u32) {
        self.write_bit(bit, 1);
    }
    pub fn reset_bit(&self, bit: u32) {
        self.write_bit(bit, 0);
    }
    pub fn get_bit(&self, bit: u32) -> u32 {
        let value = self.read() & 0b1 << bit;
        value >> bit
    }
}
