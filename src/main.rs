#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris_core::Register;

#[entry]
fn main() -> ! {
    let rcc_apb2enr = Register::new(0x4002_1000 + 0x18);
    let gpioc_crh = Register::new(0x4001_1000 + 0x04);
    let gpioc_bsrr = Register::new(0x4001_1000 + 0x10);

    rcc_apb2enr.write(0x0000_0010);
    gpioc_crh.write(0x0020_0000);
    gpioc_bsrr.write(0x0000_2000);

    loop {}
}

mod peris_core {
    use core::ptr;

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
            unsafe { ptr::write_volatile(self.address, value) }
        }
        pub fn read(&self) -> u32 {
            let value;
            unsafe {
                value = ptr::read_volatile(self.address);
            }
            value
        }
    }
}
