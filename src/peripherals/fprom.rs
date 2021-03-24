use super::flash::Flash;

pub struct Fprom {
    address: usize,
    flash: Flash,
    pages_num: usize,
    page_size: usize,
}
impl Fprom {
    pub fn new(address: usize) -> Fprom {
        Fprom {
            address,
            flash: Flash::new(),
            pages_num: 2,
            page_size: 1024,
        }
    }

    fn get_start_address(&self, buffer_len: usize) -> usize {
        let mut empty_cell_counter = 0;
        let mut first_empty_cell_address = self.address;
        let mut cell_address = first_empty_cell_address;

        while empty_cell_counter < buffer_len {

            if self.flash[ cell_address ] == 0xffff {
                if empty_cell_counter == 0 {
                    first_empty_cell_address = cell_address;
                }
                empty_cell_counter += 1;
            } else {
                empty_cell_counter = 0;
            }
        
            cell_address += 2;
            if cell_address >= self.address + self.pages_num * self.page_size {

                for page_num in 0..self.pages_num {
                    self.flash.clear_page(self.address + page_num * self.page_size);
                }

                empty_cell_counter = 0;
                first_empty_cell_address = self.address;
                cell_address = first_empty_cell_address;
            }
        }

        first_empty_cell_address
    }

    pub fn write(&mut self, buffer: &[u16]) {
        
        let address = self.get_start_address(buffer.len());

        self.flash.write(address, buffer);

    }
    fn read(&self, buffer: &[u16]) {}
}

// use core::ops::{Deref, DerefMut};
// impl Deref for Fprom {
//     type Target = Flash;

//     fn deref(&self) -> &Self::Target {
//         &self.flash
//     }
// }
// impl DerefMut for Fprom {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.flash
//     }
// }