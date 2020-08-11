use crate::core::rcc::Rcc;
use crate::core::register::Register;
pub struct Usb {
    ep0r: Register,
    cntr: Register,
    istr: Register,
    daddr: Register,
    btable: Register,
    pma: Register,
    buffer: [u32;64],
}
impl Usb {
    pub fn new() -> Usb {
        Rcc::new().enable_iopa();
        Usb::enable_clock();
        let address = 0x4000_5C00;
        Usb {
            ep0r: Register::new(address),
            cntr: Register::new(address + 0x40),
            istr: Register::new(address + 0x44),
            daddr: Register::new(address + 0x4C),
            btable: Register::new(address + 0x50),
            pma: Register::new(0x4000_6000),
            buffer: [0; 64],
        }
    }

    pub fn write_pma(&self, buffer: &[u32], mut offset: u32) {
        for element in buffer {
            let pma = (0x4000_6000 + offset * 2) as *mut u32;
            unsafe {
                *pma = *element;
            }
            offset += 1;
        }
    }
    pub fn read_pma(&self, buffer: &mut [u32], offset: u32) {
        let pma = (0x4000_6000 + offset) as *mut u32;
        for element in buffer {
            unsafe {
                *element = *pma;
            }
        }
    }

    pub fn get_correct_transfer_status(&self) -> u32 {
        self.istr.get_bit(15)
    }
    pub fn correct_transfer(&self) {
        self.istr.write_and(!0x8000);
    }

    pub fn get_pma_over_status(&self) -> u32 {
        self.istr.get_bit(14)
    }
    pub fn pma_over(&self) {
        self.istr.write_and(!0x4000);
    }

    pub fn get_err_status(&self) -> u32 {
        self.istr.get_bit(13)
    }
    pub fn err(&self) {
        self.istr.write_and(!0x2000);
    }

    pub fn get_wake_up_status(&self) -> u32 {
        self.istr.get_bit(12)
    }
    pub fn wake_up(&self) {
        self.istr.write_and(!0x1000);
    }

    pub fn get_suspend_mode_status(&self) -> u32 {
        self.istr.get_bit(11)
    }
    pub fn suspend_mode(&self) {
        self.istr.write_and(!0x0800);
    }

    pub fn get_reset_status(&self) -> u32 {
        self.istr.get_bit(10)
    }
    pub fn reset(&self) {
        let descriptor0: [u32; 4] = [64, 0, 128, 33792];
        Usb::enable_clock();
        self.write_pma(&descriptor0, 0);
        self.cntr.write(0);
        self.btable.write(0);
        self.istr.write(0);
        self.ep0r.write( 512 | 12288 & 12288 ^ self.ep0r.read() );
        self.daddr.write(0);
        self.istr.write_and( !0x0400 );
    }

    pub fn get_start_of_frame_status(&self) -> u32 {
        self.istr.get_bit(9)
    }
    pub fn start_of_frame(&self) {
        self.istr.write_and(!0x0200);
    }

    pub fn get_expected_start_of_frame_status(&self) -> u32 {
        self.istr.get_bit(8)
    }
    pub fn expected_start_of_frame(&self) {
        self.istr.write_and(!0x0100);
    }

    pub fn get_correct_transfer_for_reception_status(&self) -> u32 {
        self.ep0r.get_bit(15)
    }
    pub fn correct_transfer_for_reception(&self) {
        let mut buffer: [u32; 64] = [0; 64];
        self.read_pma(&mut buffer[..], 128);

        match buffer[0] + buffer[1] * 256 {
            0x0680 => {
                match buffer[3] {
                    0x01 => {
                        let device_descriptor: [u32; 18] = [
                            0x12, 0x01, 0x00, 0x02,
                            0x00, 0x00, 0x00, 0x40,
                            0xff, 0xff, 0xff, 0xff,
                            0x01, 0x00, 0x01, 0x02,
                            0x03, 0x01,
                        ];
                        self.write_pma(&device_descriptor, 64);
                        self.write_pma(&[buffer[3]], 2);
                        
                        self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(7) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(15) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );

                    },
                    0x02 => {
                        let config_descriptor: [u32; 41] = [
                            0x09, 0x02, 41, 0x00, 0x01, 0x01, 0x00, 0xE0, 0x32,
                            0x09, 0x04, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00,
                            0x09, 0x21, 0x01, 0x01, 0x00, 0x01, 0x22, 23, 0x00,
                            0x07, 0x05, 0x81, 0x03, 0x40, 0x00, 0x20,
                            0x07, 0x05, 0x01, 0x03, 0x40, 0x00, 0x20,
                        ];
                        self.write_pma(&config_descriptor, 64);
                        let mut buf1 = buffer[6] + buffer[7] * 256;
                        if buf1 == 9 {

                        } else {
                            buf1 = 41;
                        }
                        self.write_pma(&[buf1], 2);

                        self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(7) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(15) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                    },
                    0x03 => {
                        match buffer[2] {
                            0 => {
                                let string_descriptor = [4, 3, 9, 13];
                                self.write_pma(&string_descriptor, 64);
                                let buf1 = 4;
                                self.write_pma(&[buf1], 2);
                            },
                            _ => {
                                let string_descriptor = [10, 3, 49, 0, 49, 0, 49, 0, 49, 0];
                                self.write_pma(&string_descriptor, 64);
                                let buf1 = 10;
                                self.write_pma(&[buf1], 2);
                            },
                        }
                        self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(7) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(15) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                    },
                    _ => panic!(""),
                }
            },
            0x0500 => {
                let addr_buf = &[buffer[2] | 128];
                self.write_pma(&[0], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.daddr.write(addr_buf[0]);
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );

            },
            0x0900 => {
                self.write_pma(&[0], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
            },
            0x0A21 => {
                self.write_pma(&[0], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
            },
            0x0100 => {
                self.write_pma(&[0], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
            },
            0x0681 => {
                match buffer[3] {
                    0x22 => {
                        let report_descriptor = [
                            0x06, 0x00, 0xff,
                            0x09, 0x01,
                            0xa1, 0x01,
                            0x19, 0x01,
                            0x29, 0x01,
                            0x15, 0x00,
                            0x26, 0xff, 0x00,
                            0x75, 0x08,
                            0x95, 64,
                            0xB1, 0x02,
                            0xc0
                        ];
                        self.write_pma(&report_descriptor, 64);
                        self.write_pma(&[23], 2);
                        self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(7) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                        while self.ep0r.get_bit(15) == 0 {}
                        self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                    }
                    _ => {

                    }
                }
            },
            0x0921 => {
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                while self.ep0r.get_bit(15) == 0 {};
                self.write_pma(&[0], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
            },
            0x01A1 => {
                self.write_pma(&buffer, 64);
                self.write_pma(&[64], 2);
                self.ep0r.write( 36671 & 48 ^ self.ep0r.read() );
                while self.ep0r.get_bit(7) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
                while self.ep0r.get_bit(15) == 0 {}
                self.ep0r.write( 16271 & 12288 ^ self.ep0r.read() );
            },
            _ => panic!(""),
        }
    }

    pub fn enable_clock() {
        Rcc::new().enable_usb();
    }
    pub fn disable_clock() {
        Rcc::new().disable_usb();
    }
}