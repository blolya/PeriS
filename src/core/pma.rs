
pub struct Pma {
    pma_cell: PmaCell,
}
impl Pma {
    pub fn new() -> Pma {
        Pma {
            pma_cell: PmaCell::new(),
        }
    }
    pub fn write_u8_buffer(&mut self, buffer: &[u8], offset: usize) {
        let buffer_length = buffer.len();
    
        for buffer_index in (0..buffer_length).step_by(2) {
            let lower_byte = buffer[buffer_index];
            let upper_byte = if (buffer_length & 0b1 == 0b1) & (buffer_index == buffer_length - 1) {
                0x00
            } else {
                buffer[buffer_index + 1]
            };

            let pma_word = (upper_byte as u16) << 8 | lower_byte as u16;

            self[offset / 2 + buffer_index / 2] = pma_word;
        }
    }
    pub fn read_u8_buffer<'a>(&self, buffer: &'a mut [u8], offset: usize) -> &'a mut [u8] {
        
        for buffer_index in 0..buffer.len() {
            let pma_index = (offset & !0b1) / 2 + (buffer_index & !0b1) / 2 + ((offset & 0b1) & (buffer_index & 0b1));
            let shift = ((offset & 0b1) ^ (buffer_index & 0b1)) * 8;
            buffer[buffer_index] = (self[pma_index] >> shift) as u8;
        }
        buffer
    }

}
use core::ops::{Deref, DerefMut};
impl Deref for Pma {
    type Target = PmaCell;

    fn deref(&self) -> &Self::Target {
        &self.pma_cell
    }
}
impl DerefMut for Pma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pma_cell
    }
}
pub struct PmaCell {
    len: usize,
    address: usize,
}
impl PmaCell {
    pub fn new() -> PmaCell {
        PmaCell {
            len: 512,
            address: 0x4000_6000,
        }
    }
}

use core::ops::{Index, IndexMut};
impl Index<usize> for PmaCell {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len / 2 {
            panic!("Pma index is out of bounds");
        }

        unsafe {
            &(*((self.address + index * 4) as *const u16)) 
        }
    }
}
impl IndexMut<usize> for PmaCell {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len / 2 {
            panic!("Pma index is out of bounds");
        }

        unsafe { 
            &mut (*((self.address + index * 4) as *mut u16)) 
        }
    }
}