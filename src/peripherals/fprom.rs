use super::flash::Flash;

pub struct Fprom {
    address: usize,
    flash: Flash,
    pages_num: usize,
}
impl Fprom {
    pub fn new(address: usize) -> Fprom {
        Fprom {
            address,
            flash: Flash::new(),
            pages_num: 2,
        }
    }

    fn get_write_address(&self, buffer_len: usize) -> usize {
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
            if cell_address >= self.address + self.pages_num * self.flash.get_page_size() - buffer_len {

                self.erase();

                empty_cell_counter = 0;
                first_empty_cell_address = self.address;
                cell_address = first_empty_cell_address;
            }
        }

        first_empty_cell_address
    }

    pub fn get_read_address(&self, buffer_len: usize) -> usize {
        
        let mut non_empty_cell_counter = 0;
        let mut first_non_empty_cell_address = self.address + self.pages_num * self.flash.get_page_size();
        let mut cell_address = first_non_empty_cell_address;
        
        while non_empty_cell_counter == 0 {

            if self.flash[cell_address] != 0xffff {
                first_non_empty_cell_address = cell_address - buffer_len;
                non_empty_cell_counter = 1;
            }

            cell_address -= 2;

            if cell_address <= self.address + buffer_len {
                non_empty_cell_counter = 1;
                first_non_empty_cell_address = cell_address;
            }
        }

        first_non_empty_cell_address

    }

    pub fn write(&mut self, buffer: &[u16]) {
        
        let address = self.get_write_address(buffer.len());

        self.flash.write(address, buffer);

    }
    pub fn read(&self, buffer: &mut [u16]) {

        let address = self.get_read_address(buffer.len());
        self.flash.read(address, buffer);
    }
    pub fn erase(&self) {
        for page_num in 0..self.pages_num {
            self.flash.erase_page( self.address + page_num * self.flash.get_page_size());
        }
    }
}