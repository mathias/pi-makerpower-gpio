use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;

// GPIO pins
//const ALERT_GPIO: u8 = 4;

// I2C charger address
const MPPT_CHG_I2C_ADDR: u16 = 0x12;

//
// Internal register addresses
//
// RO values (16-bits)
const MPPT_CHG_REG_ID: u16 =  0;
const MPPT_CHG_STATUS: u16 =  2;
const MPPT_CHG_BUCK  : u16 =  4;
const MPPT_CHG_VS    : u16 =  6;
const MPPT_CHG_IS    : u16 =  8;
const MPPT_CHG_VB    : u16 =  10;
const MPPT_CHG_IB    : u16 =  12;
const MPPT_CHG_IC    : u16 =  14;
const MPPT_CHG_INT_T : u16 =  16;
const MPPT_CHG_EXT_T : u16 =  18;
const MPPT_CHG_VM    : u16 =  20;
const MPPT_CHG_TH    : u16 =  22;

// Helper functions to encode and decode binary-coded decimal (BCD) values.
fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
}

fn run_i2c() -> Result<(), Box<dyn Error>> {
    let _gpio = Gpio::new();
    let mut i2c = I2c::new()?;

    let mut reg = [0u8; 3];

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(MPPT_CHG_I2C_ADDR)?;

    println!("Hello, world!");

    loop {
        let _ = i2c.block_read(MPPT_CHG_STATUS as u8, &mut reg);

        println!("Charge status:");
        println!("{:?}", bcd2dec(reg[0]));
        println!("{:?}", bcd2dec(reg[1]));
        println!("{:?}", bcd2dec(reg[2]));


        println!("Solar voltage:");
        let _ = i2c.block_read(MPPT_CHG_VS as u8, &mut reg);

        println!("{:?}", bcd2dec(reg[0]));
        println!("{:?}", bcd2dec(reg[1]));
        println!("{:?}", bcd2dec(reg[2]));

        println!("Battery voltage:");
        let _ = i2c.block_read(MPPT_CHG_VB as u8, &mut reg);

        println!("{:?}", bcd2dec(reg[0]));
        println!("{:?}", bcd2dec(reg[1]));
        println!("{:?}", bcd2dec(reg[2]));

        thread::sleep(Duration::from_secs(1));
    }

}

fn main() {
    let _ = run_i2c();
}
