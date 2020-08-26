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

    let mut device_address = 0;

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

                usb.ep0r.write(0x0200);
                let bytes_received = unsafe {
                    *((pma_base + 12) as *mut u32) & 0xFF // allocate 64 bytes of memory for reception
                };

                let mut buffer: [u8; 64] = [0; 64];
                for i in 0..(bytes_received / 2) as usize {
                    let pma_word = unsafe {
                        *((pma_base + 128 * 2 + i * 4) as *mut u16)
                    };
                    buffer[2 * i] = (pma_word & 0xff) as u8;
                    buffer[2 * i + 1] = (pma_word >> 8 & 0xff) as u8;
                }

                dbgr.send("The");
                dbgr.send_byte((bytes_received & 0xFF) as u8);
                dbgr.send("bytes of data received. Data:");
                for element in buffer.iter() {
                    dbgr.send_byte(*element);
                }
                dbgr.send("_______________________\r\n");

                if (buffer[0] as u16) << 8 | buffer[1] as u16 == 0x8006 {
                    unsafe {
                        *((pma_base + 64 * 2) as *mut u16) = 0x0112 as u16;
                        *((pma_base + 64 * 2 + 4) as *mut u16) = 0x0200 as u16;
            
                        *((pma_base + 64 * 2 + 8) as *mut u16) = 0x0000 as u16;
                        *((pma_base + 64 * 2 + 12) as *mut u16) = 0x4000 as u16;
            
                        // *((pma_base + 64 * 2 + 16) as *mut u16) = 0xffff as u16;
                        // *((pma_base + 64 * 2 + 20) as *mut u16) = 0xffff as u16;
            
                        // *((pma_base + 64 * 2 + 24) as *mut u16) = 0x0001 as u16;
                        // *((pma_base + 64 * 2 + 28) as *mut u16) = 0x0201 as u16;
            
                        // *((pma_base + 64 * 2 + 32) as *mut u16) = 0x0103 as u16;
    
                        *((pma_base + 4) as *mut u16) = 0x08 as u16;
                        // *((pma_base + 4) as *mut u16) = 0x4000 as u16;
                    };
                    usb.ep0r.write(0x0210);
                    while usb.ep0r.get_bit(7) == 0 {};

                    usb.ep0r.write(0x1200);
                    while usb.ep0r.get_bit(15) == 0 {};

                    usb.ep0r.write(0x1200);
                }
                if (buffer[0] as u16) << 8 | buffer[1] as u16 == 0x0005 {

                    dbgr.send("Set address request");
                    dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
                    dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
                    dbgr.send("_______________________\r\n");

                    device_address = buffer[2] | 128;

                    unsafe {
                        *((pma_base + 4) as *mut u16) = 0x00 as u16;
                    };
                    usb.ep0r.write(0x0210);
                    while usb.ep0r.get_bit(7) == 0 {};
                    usb.daddr.write(device_address as u32);
                    usb.ep0r.write(0x5200);


                    dbgr.send("Address transmitted");
                    dbgr.send_byte(((usb.ep0r.read() & (0xFF << 8)) >> 8) as u8);
                    dbgr.send_byte(((usb.ep0r.read()) & 0xFF) as u8);
                    dbgr.send_byte(((usb.daddr.read()) & 0xFF) as u8);
                    dbgr.send("_______________________\r\n");
                }
            }

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