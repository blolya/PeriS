#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    communication::usb::Usb,
    communication::uart::usart1::Usart1,
    clock,
    clock::pll::Pll,
    ports::{
        Port,
        PortMode,
        PortNum,
        OutputConfig,
        MaxSpeed,
        InputConfig,
    },
};
use peris::core::{
    rcc::Rcc,
    gpio::{
        gpioa::Gpioa,
        gpioc::Gpioc
    },
};

#[entry]
fn main() -> ! {
    clock::init(); // System clock 72MHz from Pll

    let gpioa = Gpioa::new();
    let pa7 = Port::new(PortNum::P7, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz)), &gpioa);

    let dbgr = Usart1::new();
    Rcc::new().enable_iopa();
    // Rcc::new().enable_afio();

    // let pa12 = Port::new(PortNum::P12, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S50MHz)), &gpioa);
    // pa12.set_low();

    // for _ in 0..10000 {};
    // let pa11 = Port::new(PortNum::P11, PortMode::Input(InputConfig::Floating), &gpioa);
    // let pa12 = Port::new(PortNum::P12, PortMode::Input(InputConfig::Floating), &gpioa);

    Rcc::new().enable_usb();
    let usb = Usb::new();

    usb.cntr.reset_bit(1);
    for _ in 0..100000 {};

    usb.cntr.reset_bit(0);
    usb.istr.write(0);
    usb.cntr.write(0); // enable ctrm, suspm and resetm interrupts (I don't use them in this example, but enable just in case)
    usb.btable.write(0); // set table address to 0

    usb.daddr.write(0x80); // enable usb function
    let pma_base = 0x4000_6000; // pma address
    unsafe {
        // pma write/read works good
        *(pma_base as *mut u32) = 64; // writing transmission buffer address
        *((pma_base + 8) as *mut u32) = 128; // writing reception buffer address
    }
    // usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 9)) | (0x1 << 9) ); // Configure 0 endpoint to be control by writing 01 

    // unsafe {
    //     *((pma_base + 4) as *mut u32) = 64; // allocate 64 bytes of memory for transmition
    // }
    // usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 12)) | (0x3 << 12) ); // enable endpoint for reception

    unsafe {
        *((pma_base + 12) as *mut u32) = 0x8400; // allocate 64 bytes of memory for reception
    }
    // usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 4)) | (0x3 << 4) ); // enable endpoint for transmition
    usb.ep0r.write_or(0x3230);

    // dbgr.send_byte(unsafe { (usb.ep0r.read() & 0xFF << 8) >> 8 } as u8);
    // dbgr.send_byte(unsafe { (usb.ep0r.read() & 0xFF) } as u8);
    // dbgr.send_byte(unsafe { ((*((pma_base) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
    // dbgr.send_byte(unsafe { *((pma_base) as *mut u32) & 0xFF } as u8);
    // dbgr.send_byte(unsafe { ((*((pma_base + 4) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
    // dbgr.send_byte(unsafe { *((pma_base + 4) as *mut u32) & 0xFF } as u8);
    // dbgr.send_byte(unsafe { ((*((pma_base + 8) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
    // dbgr.send_byte(unsafe { *((pma_base + 8) as *mut u32) & 0xFF } as u8);
    // dbgr.send_byte(unsafe { ((*((pma_base + 12) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
    // dbgr.send_byte(unsafe { *((pma_base + 12) as *mut u32) & 0xFF } as u8);
    // dbgr.send_byte((usb.istr.read() & 0xFF) as u8);
    // dbgr.send_byte((usb.daddr.read() & 0xFF) as u8);
    // dbgr.send_byte(((usb.cntr.read() & 0xFF << 8) >> 8) as u8);
    // dbgr.send_byte((usb.cntr.read() & 0xFF) as u8);
    // dbgr.send_byte((usb.btable.read() & 0xFF) as u8);
    // dbgr.send_byte('\r' as u8);

    loop {
        // dbgr.send_byte('\r' as u8);

        // correct transfer interrupt handler
        if usb.istr.get_bit(15) == 1 {
            dbgr.send("Start:");
            dbgr.send_byte(unsafe { ((*((pma_base + 128 * 2) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
            dbgr.send_byte(unsafe { *((pma_base + 128 * 2) as *mut u32) & 0xFF } as u8);
            dbgr.send_byte(unsafe { ((*((pma_base + 128 * 2 + 4) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
            dbgr.send_byte(unsafe { *((pma_base + 128 * 2 + 4) as *mut u32) & 0xFF } as u8);
            dbgr.send_byte(unsafe { ((*((pma_base + 128 * 2 + 8) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
            dbgr.send_byte(unsafe { *((pma_base + 128 * 2 + 8) as *mut u32) & 0xFF } as u8);
            dbgr.send_byte(unsafe { ((*((pma_base + 128 * 2 + 12) as *mut u32)) & 0xFF << 8) >> 8 } as u8);
            dbgr.send_byte(unsafe { *((pma_base + 128 * 2 + 12) as *mut u32) & 0xFF } as u8);
            dbgr.send("\r\n");
            // let pma_base = 0x4000_6000;

            // let mut buffer = [0; 64];

            // for i in (128..160).step_by(2) {
            //     unsafe {
            //         let pma_left = *((pma_base + i * 2) as *mut u16) as u32;
            //         let pma_right = *((pma_base + i * 2 + 1) as *mut u16) as u32;
            //         buffer[i] = pma_left << 16 | pma_right;
            //     }
            // }
    
            // match buffer[0] + buffer[1] * 256 {
            //     0x0680 => {
            //         unsafe {
            //             *((pma_base + 0x80) as *mut u16) = 0x1201 as u16;
            //             *((pma_base + 0x84) as *mut u16) = 0x1001 as u16;
            
            //             *((pma_base + 0x88) as *mut u16) = 0x0000 as u16;
            //             *((pma_base + 0x8C) as *mut u16) = 0x0040 as u16;
            
            //             *((pma_base + 0x90) as *mut u16) = 0x8405 as u16;
            //             *((pma_base + 0x94) as *mut u16) = 0x1157 as u16;
            
            //             *((pma_base + 0x98) as *mut u16) = 0x0100 as u16;
            //             *((pma_base + 0x9C) as *mut u16) = 0x0102 as u16;
            
            //             *((pma_base + 0xA0) as *mut u16) = 0x0301 as u16;
    
            //             *((pma_base + 0x04) as *mut u16) = ((buffer[4] >> 16) & 0xFF) as u16;
            //         }
    
            //         usb.ep0r.write( 48 ^ 36671 &  usb.ep0r.read() );
                    
            //         while usb.ep0r.get_bit(7) == 0 {};
    
            //         usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
            //         while usb.ep0r.get_bit(15) == 0 {};
    
            //         usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
    
            //     },
            //     0x0000 => {
            //         // dbgr.send("Zeros\r");
            //     },
            //     _ => {
            //         // dbgr.send("Smth request\r");
            //     },
            // };
            usb.istr.reset_bit(15);
        };
        // esof interrupt handler
        if usb.istr.get_bit(8) == 1 {
            // dbgr.send("Esof\r");

            usb.istr.reset_bit(8);
        };
        // sof interrupt handler
        if usb.istr.get_bit(9) == 1 {
            // dbgr.send("Sof\r");

            usb.istr.reset_bit(9);
        };

        // reset interrupt handler
        if usb.istr.get_bit(10) == 1 {
            // dbgr.send("Reset\r");

            //  Same reset routine as before loop
            usb.daddr.write(0x80); // enable usb function
            let pma_base = 0x4000_6000; // pma address
            unsafe {
                // pma write/read works good
                *(pma_base as *mut u32) = 64; // writing transmission buffer address
                *((pma_base + 8) as *mut u32) = 128; // writing reception buffer address
            }
            usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 9)) | (0x1 << 9) ); // Configure 0 endpoint to be control by writing 01 

            unsafe {
                *((pma_base + 4) as *mut u32) = 64; // allocate 64 bytes of memory for reception
            }
            usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 4)) | (0x3 << 4) ); // enable endpoint for transmition

            unsafe {
                *((pma_base + 12) as *mut u32) = 0x8C00; // allocate 64 bytes of memory for reception
            }
            usb.ep0r.write( (usb.ep0r.read() & !(0x3 << 12)) | (0x3 << 12) ); // enable endpoint for reception
        
        };

        // suspended interrupt handler
        if usb.istr.get_bit(11) == 1 {
            // dbgr.send("Suspended\r");
            // usb.cntr.set_bit(4);
            usb.istr.reset_bit(11);
        };

        // wakeup interrupt handler
        if usb.istr.get_bit(12) == 1 {
            // dbgr.send("Wakeup\r");

            usb.istr.reset_bit(12);
        };
        // error interrupt handler
        if usb.istr.get_bit(13) == 1 {
            // dbgr.send("Err\r");

            usb.istr.reset_bit(13);
        };
        // pma over interrupt handler
        if usb.istr.get_bit(14) == 1 {
            // dbgr.send("Pma over\r");

            usb.istr.reset_bit(14);
        };

    }
}