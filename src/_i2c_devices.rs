#![allow(dead_code)]

use embassy_rp::rtc::DayOfWeek;
use embassy_rp::{bind_interrupts, rtc::{DateTime,DateTimeError}};
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::I2C0;
use embedded_hal_1::i2c::ErrorType;
use embedded_hal_async::i2c::I2c;
use log::info;
use {defmt_rtt as _, panic_probe as _};

mod ds3231;
mod stemma_soil;


// who designed this thing??!?!, you need to do some bitshift magic to get your actual value.
// ds3231 datasheet page 11 https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231.pdf
/*ds3231 registers:
    SecAddr = 0x0,
    MinAddr = 0x1,
    HourAddr = 0x2,
    DayOfTheWeekSSAddr = 0x3,
    DateAddr = 0x4,
    MonAddr = 0x5,
    YrAddr = 0x6,
    CtlAddr = 0x0e,
    StatAddr = 0x0f,
*/ 

const RTC_DEFAULT_ADDR:u8 = 0x68;
const STEMMA_DEFAULT_ADDR:u8 = 0x36;

pub struct I2cDevices<I2C:I2c>{
    ic2_bus:I2C,
}

fn day_of_week_from_u8(v: u8) -> Result<DayOfWeek, DateTimeError> {
    Ok(match v {
        0 => DayOfWeek::Sunday,
        1 => DayOfWeek::Monday,
        2 => DayOfWeek::Tuesday,
        3 => DayOfWeek::Wednesday,
        4 => DayOfWeek::Thursday,
        5 => DayOfWeek::Friday,
        6 => DayOfWeek::Saturday,
        x => return Err(DateTimeError::InvalidDayOfWeek(x)),
    })
}

impl<I2C:I2c> I2cDevices<I2C>{
    pub fn new(i2c_bus:I2C) -> Self{
        I2cDevices { 
            ic2_bus:i2c_bus }
    }

    

    

    // stemma soil sensor
    pub async fn get_moisture(&mut self) -> Result<(),<I2C as ErrorType>::Error>{
        let mut buf:[u8;2] = [0;2];
        self.ic2_bus.write_read(STEMMA_DEFAULT_ADDR, &[0x0F,0x10], &mut buf).await?;
        info!("[{:#010b},{:#010b}]",buf[0],buf[1]);
        let moisture = ((buf[0] as u16)<<8) | (buf[1] as u16);
        info!("moisture:{}", moisture);
        Ok(())
    }
}

pub trait I2cDevice{
    fn get_default_address(&self) -> u8;
    fn is_connected(&self) -> bool;
}