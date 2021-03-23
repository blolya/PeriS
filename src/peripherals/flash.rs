use super::super::core::flash::Flash as CoreFlash;

pub struct Flash {
    flash_cell: FlashCell,
}

impl Flash {
    pub fn new() -> Flash {
        Flash {
            flash_cell: FlashCell::new(),
        }
    }
    pub fn write(&mut self, address: usize, buffer: &[u16])  {

        let flash = CoreFlash::new();

        if flash.get_cr_lock() == 1 {
            flash.unlock_cr();
        }
        while flash.get_cr_lock() == 1 {};
    
        while flash.get_sr_bsy() == 1 {};
        flash.reset_eop();
    
        flash.select_programming();

        let buffer_len = buffer.len();
        for buffer_index in 0 .. buffer_len {
            self[address + buffer_index * 2] = buffer[buffer_index];
            while flash.get_eop() == 0 {};
            flash.reset_eop();
        }
    
        flash.unselect_programming();

    }
    pub fn read(&self, address: usize, buffer: &mut [u16]) {
        let buffer_len = buffer.len();
        for buffer_index in 0 .. buffer_len {
            buffer[buffer_index] = self[address + buffer_index * 2];
        }
    }
}

use core::ops::{Deref, DerefMut};
impl Deref for Flash {
    type Target = FlashCell;

    fn deref(&self) -> &Self::Target {
        &self.flash_cell
    }
}
impl DerefMut for Flash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flash_cell
    }
}

pub struct FlashCell {
    len: usize,
    address: usize,
}

impl FlashCell {
    fn new() -> FlashCell {
        FlashCell {
            len: 131072,
            address: 0x0800_0000,
        }
    }
}

use core::ops::{Index, IndexMut};
impl Index<usize> for FlashCell {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Pma index is out of bounds");
        }

        unsafe {
            &(*((self.address + index) as *const u16)) 
        }
    }
}
impl IndexMut<usize> for FlashCell {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Pma index is out of bounds");
        }

        unsafe { 
            &mut (*((self.address + index) as *mut u16)) 
        }
    }
}