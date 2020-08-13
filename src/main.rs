#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::{
    communication::usb::Usb,
    communication::uart::usart1::Usart1,
    clock,
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
    let dbgr = Usart1::new();

    Rcc::new().cfgr.reset_bit(22);
    let usb = Usb::new();
    usb.cntr.reset_bit(1);
    for _ in 0..10000 {};
    usb.cntr.write(0);
    usb.btable.write(0);
    usb.istr.write(0);
    usb.cntr.write_or(0x8C00);

    while usb.istr.get_bit(10) == 0 {};
    // usb.cntr.reset_bit(10);
    // dbgr.send("Reset has occured\r");

    usb.ep0r.write(0x3220);
    let pma_base = 0x4000_6000;
    unsafe {
        *(pma_base as *mut u16) = 64 as u16;
        *((pma_base + 8) as *mut u16) = 128 as u16;
        *((pma_base + 12) as *mut u16) = 0x8400 as u16;
    }
    usb.daddr.write(0x80);
    while usb.istr.get_bit(15) == 0 {};
    dbgr.send("Qwe\r");

    // dbgr.send("Correct transfer has occured\r");

    let dir = usb.istr.get_bit(4);
    let ep_id = usb.istr.read() & 0xF;

    if ep_id == 0 {
        while usb.ep0r.get_bit(15) == 0 {};
        // dbgr.send("0 endpoint correct transfer\r");
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
                // dbgr.send("Descriptor request\r");
                unsafe {
                    *((pma_base + 0x80) as *mut u16) = 0x1201 as u16;
                    // *((pma_base + 0x81) as *mut u16) = 0x01 as u16;
                    *((pma_base + 0x84) as *mut u16) = 0x1001 as u16;
                    // *((pma_base + 0x85) as *mut u16) = 0x01 as u16;
        
                    *((pma_base + 0x88) as *mut u16) = 0x0000 as u16;
                    // *((pma_base + 0x89) as *mut u16) = 0x00 as u16;
                    *((pma_base + 0x8C) as *mut u16) = 0x0040 as u16;
                    // *((pma_base + 0x8D) as *mut u16) = 0x40 as u16;
        
                    *((pma_base + 0x90) as *mut u16) = 0x8304 as u16;
                    // *((pma_base + 0x91) as *mut u16) = 0x04 as u16;
                    *((pma_base + 0x94) as *mut u16) = 0x1157 as u16;
                    // *((pma_base + 0x95) as *mut u16) = 0x57 as u16;
        
                    *((pma_base + 0x98) as *mut u16) = 0x0100 as u16;
                    // *((pma_base + 0x99) as *mut u16) = 0x00 as u16;
                    *((pma_base + 0x9C) as *mut u16) = 0x0102 as u16;
                    // *((pma_base + 0x9D) as *mut u16) = 0x02 as u16;
        
                    *((pma_base + 0xA0) as *mut u16) = 0x0301 as u16;
                    // *((pma_base + 0xA1) as *mut u16) = 0x01 as u16;

                    *((pma_base + 0x04) as *mut u16) = ((buffer[4] >> 16) & 0xFF) as u16;
                }

                usb.ep0r.write( 48 ^ 36671 &  usb.ep0r.read() );
                
                while usb.ep0r.get_bit(7) == 0 {};
                // dbgr.send("Correct descriptor transmission\r");

                usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
                while usb.ep0r.get_bit(15) == 0 {};
                // dbgr.send("Empty package\r");

                usb.ep0r.write( 12288 ^ 16271 &  usb.ep0r.read() );
                // dbgr.send("Empty package\r");

            },
            0x0000 => {
                // dbgr.send("Error\r");
            },
            _ => {
                // dbgr.send("Smth request\r");
            },
        };
    }

    // usb.reset();

    loop {
        // if usb.istr.get_bit(10) == 1 {
        //     usb.cntr.reset_bit(10);
        // }

        // if usb.get_correct_transfer_status() == 1 {
        //     usb.correct_transfer();
        // }
        // if usb.get_pma_over_status() == 1 {
        //     usb.pma_over();
        // }
        // if usb.get_err_status() == 1 {
        //     usb.err();
        // }
        // if usb.get_wake_up_status() == 1 {
        //     usb.wake_up();
        // }
        // if usb.get_suspend_mode_status() == 1 {
        //     usb.suspend_mode();
        // }
        // if usb.get_reset_status() == 1 {
        //     usb.reset();
        // }
        // if usb.get_start_of_frame_status() == 1 {
        //     usb.start_of_frame();
        // }
        // if usb.get_expected_start_of_frame_status() == 1 {
        //     usb.expected_start_of_frame();
        // }
        // if usb.get_correct_transfer_for_reception_status() == 1 {
        //     dbgr.send("Qqwe\r");
        //     // usb.correct_transfer_for_reception();
        // }
    }
}
