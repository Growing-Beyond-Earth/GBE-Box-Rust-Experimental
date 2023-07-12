#![allow(dead_code)]

use embassy_rp::rtc::DayOfWeek;
use embassy_rp::{bind_interrupts, rtc::{DateTime,DateTimeError}};
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::I2C0;
use embedded_hal_1::i2c::ErrorType;
use embedded_hal_async::i2c::I2c;
use log::info;
use {defmt_rtt as _, panic_probe as _};



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

const DEFAULT_I2C_ADDR:u8 = 0x68;

pub struct Ds3231<I2C:I2c>{
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

impl<I2C:I2c> Ds3231<I2C>{
    pub fn new(i2c_bus:I2C) -> Self{
        Ds3231 { 
            ic2_bus:i2c_bus }
    }

    pub async fn date_time(&mut self) -> Result<DateTime,<I2C as ErrorType>::Error>{
        let mut buf:[u8;19] = [0;19];
        self.ic2_bus.read(DEFAULT_I2C_ADDR, &mut buf).await?;
        info!("{:?}",buf);
        buf.rotate_left(4);
        // do some bitshift magic to convert.
        return Ok(DateTime{
            year:(((buf[0x06] & 0b00001111) + ((buf[0x06] >> 4)*10)) as u16) + ((((buf[0x05] >> 7) as u16)*1000u16) +1000u16), // yeah, i know
            month: (buf[0x05] & 0b00001111) + ((buf[0x05] & 0b00010000) >> 4) * 10,
            day: ((buf[0x04] & 0b00001111) + (buf[0x04] >> 4)*10),
            day_of_week: day_of_week_from_u8(buf[0x03]-1).unwrap(),
            hour: ((buf[0x02] & 0b00001111) + ((buf[0x02] & 0b00010000) >> 4)*10),
            minute: ((buf[0x01] & 0b00001111) + (buf[0x01] >> 4)*10),
            second: ((buf[0x00] & 0b00001111) + (buf[0x00] >> 4)*10),
        });
    }

    pub async fn set_time(&mut self, dt:DateTime) -> Result<(),<I2C as ErrorType>::Error>{
        // unpack the datetime
        let DateTime{year, month, day, day_of_week, hour, minute, second} = dt;
        // numbers are for proper configurations
        let buf =[
            136,
            0,
            24,
            ( ((second/10) << 4) | (second-(second/10)) ),
            

        ];
        self.ic2_bus.write(DEFAULT_I2C_ADDR, &buf).await?;
        Ok(())
    }
}