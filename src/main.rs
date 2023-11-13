use std::error::Error;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;

const ALERT_GPIO: u8 = 4;

// I2C setup
const MPPT_CHG_I2C_ADDR: u16 = 0x12;
const MPPT_CHG_REG_ID: usize = 0;

fn run_i2c() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new();
    let mut i2c = I2c::new()?;

    let mut reg = [0u8; 3];

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(MPPT_CHG_I2C_ADDR)?;

    println!("Hello, world!");

    loop {
        let _ = i2c.block_read(MPPT_CHG_REG_ID as u8, &mut reg);
        println!("{:?}", reg);
    }

}

fn main() {
    let _ = run_i2c();
}
