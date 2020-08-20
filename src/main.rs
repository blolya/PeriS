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

    // System and power-on reset r.m. p.626
    usb.cntr.reset_bit(1);
    for _ in 0..100000 {};
    usb.cntr.reset_bit(0);
    usb.istr.write(0);
    usb.btable.write(0);

    // Usb reset (RESET interrupt) p.626
    usb.daddr.write(0x80);
    // Endpoint initialization p. 628
    let pma_base = 0x4000_6000;
    unsafe {
        *(pma_base as *mut u32) = 64;
        *((pma_base + 8) as *mut u32) = 128;
    }

    usb.ep0r.write(0x3200);
    unsafe {
        *((pma_base + 12) as *mut u32) = 0x8400;
    }



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

        // correct transfer interrupt handler
        if usb.istr.get_bit(15) == 1 {
            dbgr.send("Correct transfer");
            let ep_id = usb.istr.read() & 0xF;
            let dir = usb.istr.get_bit(4);

            dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            dbgr.send_byte((dir & 0xFF) as u8);
            dbgr.send_byte((usb.ep0r.get_bit(11) & 0xFF) as u8);
            dbgr.send("_______________________\r\n");

            if ep_id == 0 {
                let transaction_type = usb.ep0r.get_bit(11);
                usb.ep0r.write(0x0280);
                let bytes_received = unsafe {
                    *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
                };

                let mut buffer = [0; 8];
                for i in 0..(bytes_received / 4) as usize {
                    buffer[i] = unsafe {
                        let pma_first_word = *((pma_base + 128 * 2 + i * 8) as *mut u16);
                        let pma_second_word = *((pma_base + 128 * 2 + i * 8 + 4) as *mut u16);

                        let left_part = (pma_first_word & 0xff) << 8 | (pma_first_word >> 8) & 0xff;
                        let right_part = (pma_second_word & 0xff) << 8 | (pma_second_word >> 8) & 0xff;

                        (left_part as u32) << 16 | right_part as u32
                    }
                }

                for element in buffer.iter() {
                    dbgr.send_byte( ((element >> 24) & 0xFF) as u8 );
                    dbgr.send_byte( ((element >> 16) & 0xFF) as u8 );
                    dbgr.send_byte( ((element >> 8) & 0xFF) as u8 );
                    dbgr.send_byte( (element & 0xFF) as u8 );
                }
                dbgr.send("_______________________\r\n");
                unsafe {
                    *((pma_base + 64 * 2) as *mut u16) = 0x1201 as u16;
                    *((pma_base + 64 * 2 + 4) as *mut u16) = 0x1001 as u16;
        
                    *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
                    *((pma_base + 64 * 2 + 12) as *mut u16) = 0x0040 as u16;
        
                    // *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
                    // *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
        
                    // *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
                    // *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
        
                    // *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;

                    *((pma_base + 4) as *mut u16) = 8 as u16;
                    // *((pma_base + 4) as *mut u16) = 4 as u16;
                }

                usb.ep0r.write(0x2210);

            }

            // if dir == 1 {
            //     usb.ep0r.write(0x0200);
                
            //     let bytes_received = unsafe {
            //         *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
            //     };
            //     let a = unsafe { ((*((pma_base + 128 * 2) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2) as *mut u32) & 0xFF };
            //     let b = unsafe { ((*((pma_base + 128 * 2 + 4) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 4) as *mut u32) & 0xFF };
            //     let c = unsafe { ((*((pma_base + 128 * 2 + 8) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 8) as *mut u32) & 0xFF };
            //     let d = unsafe { ((*((pma_base + 128 * 2 + 12) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 12) as *mut u32) & 0xFF };

            //     unsafe {
            //         *((pma_base + 64 * 2) as *mut u16) = 0x1201 as u16;
            //         *((pma_base + 64 * 2 + 4) as *mut u16) = 0x1001 as u16;
        
            //         *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
            //         *((pma_base + 64 * 2 + 12) as *mut u16) = 0x0040 as u16;
        
            //         *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
            //         *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
        
            //         *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
            //         *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
        
            //         *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;

            //         *((pma_base + 4) as *mut u16) = (d & 0xFF) as u16;
            //         // *((pma_base + 4) as *mut u16) = 4 as u16;

            //     }
            //     usb.ep0r.write(0x5250);
            // }
            // if dir == 0 {
            //     unsafe {
            //         *((pma_base + 64 * 2) as *mut u16) = 0x1201 as u16;
            //         *((pma_base + 64 * 2 + 4) as *mut u16) = 0x1001 as u16;
        
            //         *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
            //         *((pma_base + 64 * 2 + 12) as *mut u16) = 0x0040 as u16;
        
            //         *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
            //         *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
        
            //         *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
            //         *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
        
            //         *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;

            //         *((pma_base + 4) as *mut u16) = 18 as u16;
            //         // *((pma_base + 4) as *mut u16) = 4 as u16;

            //     }
            //     usb.ep0r.write(0x3250);
            // }

            // if ep_id == 0 {
            //     if dir == 1 {
            //         if usb.ep0r.get_bit(15) == 1 {
            //             let setup = usb.ep0r.get_bit(11);
            //             if setup == 1 {
            //                 dbgr.send("Heeeeeee");
            //                 // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                 // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //                 let value = usb.ep0r.read() & 0x0F8F;
            //                 usb.ep0r.write(value);
            //                 // for i in 0..100000 {};
            //                 // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                 // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //                 let bytes_received = unsafe {
            //                     *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
            //                 };
            //                 let a = unsafe { ((*((pma_base + 128 * 2) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2) as *mut u32) & 0xFF };
            //                 let b = unsafe { ((*((pma_base + 128 * 2 + 4) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 4) as *mut u32) & 0xFF };
            //                 let c = unsafe { ((*((pma_base + 128 * 2 + 8) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 8) as *mut u32) & 0xFF };
            //                 let d = unsafe { ((*((pma_base + 128 * 2 + 12) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 12) as *mut u32) & 0xFF };
    
            //                 dbgr.send_byte( ((a & (0xFF << 8)) >> 8) as u8 );
            //                 dbgr.send_byte( (a & 0xFF) as u8 );
            //                 dbgr.send_byte( ((b & (0xFF << 8)) >> 8) as u8 );
            //                 dbgr.send_byte( (b & 0xFF) as u8 );
            //                 dbgr.send_byte( ((c & (0xFF << 8)) >> 8) as u8 );
            //                 dbgr.send_byte( (c & 0xFF) as u8 );
            //                 dbgr.send_byte( ((d & (0xFF << 8)) >> 8) as u8 );
            //                 dbgr.send_byte( (d & 0xFF) as u8 );
    

            //                 unsafe {
            //                     *((pma_base + 64 * 2) as *mut u16) = 0x1201 as u16;
            //                     *((pma_base + 64 * 2 + 4) as *mut u16) = 0x1001 as u16;
                    
            //                     *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
            //                     *((pma_base + 64 * 2 + 12) as *mut u16) = 0x0040 as u16;
                    
            //                     *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
            //                     *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
                    
            //                     *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
            //                     *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
                    
            //                     *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;

            //                     *((pma_base + 4) as *mut u16) = (d & 0xFF) as u16;
            //                 }

            //                 let mut value = usb.ep0r.read();
            //                 value = value ^ 48;
            //                 value = value & 36671;
            //                 usb.ep0r.write(value);
            //                 dbgr.send("000000000000000\r\n");

            //                 // let r_w_value = (usb.ep0r.read() & 0x0F0F);
            //                 // let toggle_value = (usb.ep0r.read() ^ 0x0030);
            //                 // usb.ep0r.write(r_w_value | toggle_value);
            //                 // usb.ep0r.write_xor(0x0230);
            //                 // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                 // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //                 while usb.ep0r.get_bit(7) == 0 {
            //                     dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                     dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //                     dbgr.send("\r\n");
            //                 };

            //                 dbgr.send("000000000000000\r\n");
                            
            //                 // usb.ep0r.write_xor(0x3200);
            //                 // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                 // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //                 let mut value = usb.ep0r.read();
            //                 value = value ^ 12288;
            //                 value = value & 16271;
            //                 usb.ep0r.write(value);
            //                 while usb.ep0r.get_bit(15) == 0 {};

            //                 // usb.ep0r.write_xor(0x3200);
            //                 // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //                 // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);

            //                 let mut value = usb.ep0r.read();
            //                 value = value ^ 12288;
            //                 value = value & 16271;
            //                 usb.ep0r.write(value);

            //                 dbgr.send("\r\n");
            //             }
            //         }
            //     }
            //     // if dir == 0 {
            //     //     if usb.ep0r.get_bit(7) == 1 {
            //     //         if a == 0x0680 {
            //     //             if (b & (0xFF << 8)) >> 8 == 0x01 {
            //     //                 dbgr.send("Yoooooo");
    
            //     //                 usb.ep0r.write(usb.ep0r.read() & !0x70F0);
            //     //                 dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //     //                 dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
    
            //     //                 if status == 0 {
            //     //                     unsafe {
            //     //                         *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
            //     //                         *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
                            
            //     //                         *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
            //     //                         *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
                            
            //     //                         // *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;
        
            //     //                         *((pma_base + 4) as *mut u16) = 8 as u16;
            //     //                         // *((pma_base + 4) as *mut u16) = 8 as u16;
            //     //                     }
            //     //                     status = 1;
            //     //                 } else {
            //     //                     unsafe {
            //     //                         *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;
        
            //     //                         *((pma_base + 4) as *mut u16) = 2 as u16;
            //     //                     }
            //     //                 }

        
            //     //                 let mut value = usb.ep0r.read() ^ 0x0030;
            //     //                 value = value & !0x7000;
            //     //                 usb.ep0r.write(value);
            //     //                 dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //     //                 dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
    
            //     //                 dbgr.send("\r\n");
    
            //     //             }
            //     //             if (b & (0xFF << 8)) >> 8 == 0x02 {
            //     //                 let g = if d == 9 {
            //     //                     9
            //     //                 } else {
            //     //                     41
            //     //                 };
            //     //                 unsafe {
            //     //                     *((pma_base + 4) as *mut u16) = g as u16;
            //     //                 }
    
                                
            //     //                 usb.ep0r.write(0x0200);
            //     //                 usb.ep0r.write(0x6250);
            //     //                 while usb.ep0r.get_bit(7) == 0 {};
            //     //                 usb.ep0r.write(0x3220);
            //     //             }
            //     //             if (b & (0xFF << 8)) >> 8 == 0x03 {
            //     //                 if b & 0xFF == 0x0 {
            //     //                     unsafe {
            //     //                         *((pma_base + 64 * 2) as *mut u16) = 0x0403 as u16;
            //     //                         *((pma_base + 64 * 2 + 4) as *mut u16) = 0x090D as u16;
            //     //                         *((pma_base + 4) as *mut u16) = 0x04 as u16;
            //     //                     }
            //     //                 } else {
            //     //                     unsafe {
            //     //                         *((pma_base + 64 * 2) as *mut u16) = 0x0A03 as u16;
            //     //                         *((pma_base + 64 * 2 + 4) as *mut u16) = 0x3300 as u16;
                            
            //     //                         *((pma_base + 64 * 2 + 8) as *mut u16) = 0x3300 as u16;
            //     //                         *((pma_base + 64 * 2 + 12) as *mut u16) = 0x3300 as u16;
                            
            //     //                         *((pma_base + 64 * 2 + 16) as *mut u16) = 0x3300 as u16;
    
            //     //                         *((pma_base + 4) as *mut u16) = 0x0A as u16;
            //     //                     }
            //     //                 }
    
            //     //                 usb.ep0r.write(0x0200);
            //     //                 usb.ep0r.write(0x6250);
            //     //                 while usb.ep0r.get_bit(7) == 0 {};
            //     //                 usb.ep0r.write(0x3220);
            //     //             }
            //     //         }
            //     //     }
            //     // }
            // }
        };
        // esof interrupt handler
        if usb.istr.get_bit(8) == 1 {
            dbgr.send("Esof interrupt\r\n");

            usb.istr.reset_bit(8);
        };
        // sof interrupt handler
        if usb.istr.get_bit(9) == 1 {
            dbgr.send("Sof interrupt\r\n");

            usb.istr.reset_bit(9);
        };

        // reset interrupt handler
        if usb.istr.get_bit(10) == 1 {
            dbgr.send("Reset interrupt\r\n");

            usb.istr.reset_bit(10);

            // Usb reset (RESET interrupt) p.626
            usb.daddr.write(0x80);
            // Endpoint initialization p. 628
            let pma_base = 0x4000_6000;
            unsafe {
                *(pma_base as *mut u32) = 64;
                *((pma_base + 8) as *mut u32) = 128;
            }

            usb.ep0r.write(0xb280);
            unsafe {
                *((pma_base + 12) as *mut u32) = 0x8400;
            }
        };

        // suspended interrupt handler
        if usb.istr.get_bit(11) == 1 {
            dbgr.send("Suspended interrupt\r\n");
            // usb.cntr.set_bit(4);
            usb.istr.reset_bit(11);
        };

        // wakeup interrupt handler
        if usb.istr.get_bit(12) == 1 {
            dbgr.send("Wakeup interrupt\r\n");

            usb.istr.reset_bit(12);
        };
        // error interrupt handler
        if usb.istr.get_bit(13) == 1 {
            dbgr.send("Err interrupt\r\n");

            usb.istr.reset_bit(13);
        };
        // pma over interrupt handler
        if usb.istr.get_bit(14) == 1 {
            dbgr.send("Pma overrun interrupt\r\n");

            usb.istr.reset_bit(14);
        };

    }
}