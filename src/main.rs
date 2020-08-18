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
    unsafe {
        *((pma_base + 12) as *mut u32) = 0x8400; // allocate 64 bytes of memory for reception
    }
    usb.ep0r.write_xor(0x3230);

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

            dbgr.send("\r\n");
            if ep_id == 0 {
                if usb.ep0r.get_bit(15) == 1 {
                    let setup = usb.ep0r.get_bit(11);
                    if setup == 1 {
                        usb.ep0r.write(usb.ep0r.read() & !0xF070);
                        let bytes_received = unsafe {
                            *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
                        };
                        let a = unsafe { ((*((pma_base + 128 * 2) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2) as *mut u32) & 0xFF };
                        let b = unsafe { ((*((pma_base + 128 * 2 + 4) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 4) as *mut u32) & 0xFF };
                        let c = unsafe { ((*((pma_base + 128 * 2 + 8) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 8) as *mut u32) & 0xFF };
                        let d = unsafe { ((*((pma_base + 128 * 2 + 12) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 12) as *mut u32) & 0xFF };

                        let value = usb.ep0r.read() & !0x4070;
                        usb.ep0r.write(value ^ 0x3000);
                    }                  
                }
            }

            // if ep_id == 0x0 {
            //     while usb.ep0r.get_bit(15) == 0 {};

            //     let bytes_received = unsafe {
            //         *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
            //     };

            //     let a = unsafe { ((*((pma_base + 128 * 2) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2) as *mut u32) & 0xFF };
            //     let b = unsafe { ((*((pma_base + 128 * 2 + 4) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 4) as *mut u32) & 0xFF };
            //     let c = unsafe { ((*((pma_base + 128 * 2 + 8) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 8) as *mut u32) & 0xFF };
            //     let d = unsafe { ((*((pma_base + 128 * 2 + 12) as *mut u32)) & 0xFF << 8) | *((pma_base + 128 * 2 + 12) as *mut u32) & 0xFF };

            //     // dbgr.send_byte((a >> 8 & 0xFF) as u8);
            //     // dbgr.send_byte((a & 0xFF) as u8);
            //     // dbgr.send_byte((b >> 8 & 0xFF) as u8);
            //     // dbgr.send_byte((b & 0xFF) as u8);
            //     // dbgr.send_byte((c >> 8 & 0xFF) as u8);
            //     // dbgr.send_byte((c & 0xFF) as u8);
            //     // dbgr.send_byte((d >> 8 & 0xFF) as u8);
            //     // dbgr.send_byte((d & 0xFF) as u8);
            //     // dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
            //     // dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
            //     // dbgr.send_byte(((b & (0xFF << 8)) >> 8) as u8);

            //     if a == 0x0680 {
            //         if (b & (0xFF << 8)) >> 8 == 0x01 {

            //             unsafe {
            //                 *((pma_base + 64 * 2) as *mut u16) = 0x1201 as u16;
            //                 *((pma_base + 64 * 2 + 4) as *mut u16) = 0x1001 as u16;
                
            //                 *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
            //                 *((pma_base + 64 * 2 + 12) as *mut u16) = 0x0040 as u16;
                
            //                 *((pma_base + 64 * 2 + 16) as *mut u16) = 0x8405 as u16;
            //                 *((pma_base + 64 * 2 + 20) as *mut u16) = 0x1157 as u16;
                
            //                 *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0100 as u16;
            //                 *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0102 as u16;
                
            //                 *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0301 as u16;

            //                 *((pma_base + 4) as *mut u16) = (d & 0xFF) as u16;
            //             }
 
            //             usb.ep0r.write(0x0200);
            //             usb.ep0r.write(0x6250);
            //             while usb.ep0r.get_bit(7) == 0 {};
            //             usb.ep0r.write(0x3220);

            //         }
            //         if (b & (0xFF << 8)) >> 8 == 0x02 {
            //             let g = if d == 9 {
            //                 9
            //             } else {
            //                 41
            //             };
            //             unsafe {
            //                 *((pma_base + 4) as *mut u16) = g as u16;
            //             }

                         
            //             usb.ep0r.write(0x0200);
            //             usb.ep0r.write(0x6250);
            //             while usb.ep0r.get_bit(7) == 0 {};
            //             usb.ep0r.write(0x3220);
            //         }
            //         if (b & (0xFF << 8)) >> 8 == 0x03 {
            //             if b & 0xFF == 0x0 {
            //                 unsafe {
            //                     *((pma_base + 64 * 2) as *mut u16) = 0x0403 as u16;
            //                     *((pma_base + 64 * 2 + 4) as *mut u16) = 0x090D as u16;
            //                     *((pma_base + 4) as *mut u16) = 0x04 as u16;
            //                 }
            //             } else {
            //                 unsafe {
            //                     *((pma_base + 64 * 2) as *mut u16) = 0x0A03 as u16;
            //                     *((pma_base + 64 * 2 + 4) as *mut u16) = 0x3300 as u16;
                    
            //                     *((pma_base + 64 * 2 + 8) as *mut u16) = 0x3300 as u16;
            //                     *((pma_base + 64 * 2 + 12) as *mut u16) = 0x3300 as u16;
                    
            //                     *((pma_base + 64 * 2 + 16) as *mut u16) = 0x3300 as u16;

            //                     *((pma_base + 4) as *mut u16) = 0x0A as u16;
            //                 }
            //             }

            //             usb.ep0r.write(0x0200);
            //             usb.ep0r.write(0x6250);
            //             while usb.ep0r.get_bit(7) == 0 {};
            //             usb.ep0r.write(0x3220);
            //         }
            //     }
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

            //  Same reset routine as before loop
            usb.daddr.write(0x80); // enable usb function
            let pma_base = 0x4000_6000; // pma address
            unsafe {
                // pma write/read works good
                *(pma_base as *mut u32) = 64; // writing transmission buffer address
                *((pma_base + 8) as *mut u32) = 128; // writing reception buffer address
            }
            unsafe {
                *((pma_base + 12) as *mut u32) = 0x8400; // allocate 64 bytes of memory for reception
            }

            usb.ep0r.write_xor(0x3230);
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