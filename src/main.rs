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
    clock::init();
    // let gpioc = Gpioc::new();
    // let pc13 = Port::new(PortNum::P13, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S2MHz)), &gpioc);
    // pc13.set_high();
    Rcc::new().enable_iopb();
    let dbgr = Usart1::new();

    let usb = Usb::new();
    usb.cntr.reset_bit(1);
    for _ in 0..10000 {};
    usb.cntr.write(0);
    usb.btable.write(0);
    usb.istr.write(0);

    while usb.istr.get_bit(10) == 0 {};
    usb.istr.reset_bit(10);
    usb.ep0r.write(512 | 12288 & 12288 ^ usb.ep0r.read());

    usb.daddr.write(0x80);

    loop {
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
            
            Rcc::new().enable_usb();
            // let pma_base = 0x4000_6000;
            let pma_base = 0x1000_1800;
            unsafe {
                *(pma_base as *mut u32) = 64;
                *((pma_base + 4) as *mut u32) = 0;
                *((pma_base + 8) as *mut u32) = 128;
                *((pma_base + 12) as *mut u32) = 0x8400;
            }

            usb.cntr.write(0);
            usb.btable.write(0);
            usb.istr.write(0);

            usb.ep0r.write( 512 | 12288 & 12288 ^ usb.ep0r.read() );
            usb.daddr.write(0x80);

            usb.istr.reset_bit(10);
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

        // correct transfer interrupt handler
        if usb.istr.get_bit(15) == 1 {
            // dbgr.send("CT\r");
            let pma_base = 0x4000_6000;
            let dir = usb.istr.get_bit(4);
            let ep_id = usb.istr.read() & 0xF;
            if ep_id == 0 {
                while usb.ep0r.get_bit(15) == 0 {};
                if dir == 1 {
                    usb.ep0r.reset_bit(15);
                }
                if dir == 0 {
                    usb.ep0r.reset_bit(7);
                }
                
                let mut buffer = [0; 64];
        
                for i in (128..160).step_by(2) {
                    unsafe {
                        let pma_left = *((pma_base + i * 2) as *mut u16) as u32;
                        let pma_right = *((pma_base + i * 2 + 1) as *mut u16) as u32;
                        buffer[i] = pma_left << 16 | pma_right;
                    }
                }
        
                match buffer[0] + buffer[1] * 256 {
                    0x0680 => {
                        unsafe {
                            *((pma_base + 0x80) as *mut u16) = 0x1201 as u16;
                            *((pma_base + 0x84) as *mut u16) = 0x1001 as u16;
                
                            *((pma_base + 0x88) as *mut u16) = 0x0000 as u16;
                            *((pma_base + 0x8C) as *mut u16) = 0x0040 as u16;
                
                            *((pma_base + 0x90) as *mut u16) = 0x8304 as u16;
                            *((pma_base + 0x94) as *mut u16) = 0x1157 as u16;
                
                            *((pma_base + 0x98) as *mut u16) = 0x0100 as u16;
                            *((pma_base + 0x9C) as *mut u16) = 0x0102 as u16;
                
                            *((pma_base + 0xA0) as *mut u16) = 0x0301 as u16;
        
                            *((pma_base + 0x04) as *mut u16) = ((buffer[4] >> 16) & 0xFF) as u16;
                        }
        
                        usb.ep0r.write( 48 ^ 36671 &  usb.ep0r.read() );
                        
                        while usb.ep0r.get_bit(7) == 0 {};
        
                        usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
                        while usb.ep0r.get_bit(15) == 0 {};
        
                        usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
        
                    },
                    0x0000 => {
                        // dbgr.send("Zeros\r");
                    },
                    _ => {
                        // dbgr.send("Smth request\r");
                    },
                };
            }
            usb.istr.reset_bit(15);
        };

        if usb.ep0r.get_bit(15) == 1 {
            dbgr.send("Ep0r\r");

            // let pma_base = 0x4000_6000;
            let pma_base = 0x1000_1800;
            let mut buffer = [0; 64];
    
            for i in (128..192).step_by(2) {
                unsafe {
                    let pma_left = *((pma_base + i * 2) as *mut u32);
                    let pma_right = *((pma_base + i * 2 + 4) as *mut u32);
                    buffer[i] = pma_left << 16 | pma_right;
                    // let pma = *((pma_base + i * 2) as *mut u32);
                    // buffer[i] = pma;
                }
            }
            match buffer[0] + buffer[1] * 256 {
                0x0680 => {
                    unsafe {
                        *((pma_base + 0x80) as *mut u16) = 0x1201 as u16;
                        *((pma_base + 0x84) as *mut u16) = 0x1001 as u16;
            
                        *((pma_base + 0x88) as *mut u16) = 0x0000 as u16;
                        *((pma_base + 0x8C) as *mut u16) = 0x0040 as u16;
            
                        *((pma_base + 0x90) as *mut u16) = 0x8304 as u16;
                        *((pma_base + 0x94) as *mut u16) = 0x1157 as u16;
            
                        *((pma_base + 0x98) as *mut u16) = 0x0100 as u16;
                        *((pma_base + 0x9C) as *mut u16) = 0x0102 as u16;
            
                        *((pma_base + 0xA0) as *mut u16) = 0x0301 as u16;
    
                        *((pma_base + 0x04) as *mut u16) = ((buffer[4] >> 16) & 0xFF) as u16;
                    }
    
                    usb.ep0r.write( 48 ^ 36671 &  usb.ep0r.read() );
                    
                    while usb.ep0r.get_bit(7) == 0 {};
    
                    usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
                    while usb.ep0r.get_bit(15) == 0 {};
    
                    usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
    
                },
                0x0000 => {
                    // dbgr.send("Zeros\r");
                },
                _ => {
                    // dbgr.send("Smth request\r");
                },
            };
        }
    }
}