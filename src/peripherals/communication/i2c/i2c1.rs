use super::super::super::{
    clock::apb1::Apb1,
    ports::{
        Port,
        PortMode,
        MaxSpeed,
        PortNum,
        OutputConfig,
    },
    super::{
        core::rcc::Rcc,
        core::register::Register,
        core::gpio::gpiob::Gpiob,
    },
};

pub struct I2c1 {
    cr1: Register,
    cr2: Register,
    oar1: Register,
    oar2: Register,
    dr: Register,
    sr1: Register,
    sr2: Register,
    ccr: Register,
    trise: Register,
}
impl I2c1 {
    pub fn new() -> I2c1 {
        Rcc::new().enable_afio();
        I2c1::enable_clock();
        I2c1::configure_ports();

        let address = 0x4000_5400;
        let i2c = I2c1 {
            cr1: Register::new(address),
            cr2: Register::new(address + 0x04),
            oar1: Register::new(address + 0x08),
            oar2: Register::new(address + 0x0c),
            dr: Register::new(address + 0x10),
            sr1: Register::new(address + 0x14),
            sr2: Register::new(address + 0x18),
            ccr: Register::new(address + 0x1c),
            trise: Register::new(address + 0x20),
        };
        i2c.set_mode_i2c();
        i2c.set_freq();
        i2c.set_half_duty();
        i2c.set_standart_mode();
        i2c.set_ccr();
        i2c.set_trise();
        i2c.start();

        i2c
    }
    pub fn read(&self, address: u32) -> u8 {
        
        self.cr1.set_bit(8);
        while self.sr1.get_bit(0) == 0 {};
        self.sr1.read();
    
        self.dr.write(0x29 << 1);
        while self.sr1.get_bit(1) == 0 {};
        self.sr1.read();
        self.sr2.read();
    
        self.dr.write(((address >> 8) & 0xff) as u32);
        while self.sr1.get_bit(7) == 0 {};
        self.dr.write((address & 0xff) as u32);
        while self.sr1.get_bit(7) == 0 {};
        self.cr1.set_bit(9);	
    
        self.cr1.set_bit(8);
        while self.sr1.get_bit(0) == 0 {};
        self.sr1.read();
    
        self.dr.write((0x29 << 1) + 1);
        while self.sr1.get_bit(1) == 0 {};
        self.sr1.read();
        self.sr2.read();
            
        self.cr1.write_and(!0x0000_0400);
        while self.sr1.get_bit(6) == 0 {};
        let data: u8 = self.dr.read() as u8;			
        self.cr1.set_bit(9);	
    
        data
    }
    pub fn write(&self, address: u32, data: u8) {
        self.cr1.set_bit(8);
        while self.sr1.get_bit(0) == 0 {};
        self.sr1.read();
        
        self.dr.write(0x29 << 1);
        while self.sr1.get_bit(1) == 0 {};
        self.sr1.read();
        self.sr2.read();
        
        self.dr.write(((address >> 8) & 0xff) as u32);
        while self.sr1.get_bit(7) == 0 {};
        self.dr.write((address & 0xff) as u32);
        while self.sr1.get_bit(7) == 0 {};
        
        self.dr.write(data as u32);
        while self.sr1.get_bit(2) == 0 {};
        self.cr1.set_bit(9);	
    }
    fn start(&self) {
        self.cr1.set_bit(0);
    }
    fn set_mode_i2c(&self) {
        self.cr1.reset_bit(1);
    }
    fn set_freq(&self) {
        let apb1_freq = Apb1::new().get_output_frequency();
        self.cr2.write_and(!0x0000_003f);
        self.cr2.write_or(apb1_freq);
    }
    fn set_standart_mode(&self) {
        self.ccr.reset_bit(15);
    }
    fn set_half_duty(&self) {
        self.ccr.reset_bit(14);
    }
    fn set_ccr(&self) {
        let apb1_freq = Apb1::new().get_output_frequency();
        self.ccr.write_and(!0x0000_0fff);
        self.ccr.write_or(apb1_freq * 5);
    }
    fn set_trise(&self) {
        let apb1_freq = Apb1::new().get_output_frequency();
        let trise = 1000 / (1000 / apb1_freq);
        self.trise.write(trise);
    }

    fn configure_ports() {
        let gpiob = Gpiob::new();

        Port::new(PortNum::P6, PortMode::Output(OutputConfig::AlternativeFunctionOpenDrain(MaxSpeed::S2MHz)), &gpiob);
        Port::new(PortNum::P7, PortMode::Output(OutputConfig::AlternativeFunctionOpenDrain(MaxSpeed::S2MHz)), &gpiob);
    }
    pub fn enable_clock() {
        Rcc::new().enable_i2c1();
    }
    pub fn disable_clock() {
        Rcc::new().disable_i2c1();
    }
}