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

    pub async fn date_time(&mut self) -> Result<DateTime,<I2C as ErrorType>::Error>{
        let mut buf:[u8;7] = [0;7];
        self.ic2_bus.write_read(RTC_DEFAULT_ADDR,&[0], &mut buf).await?;
        info!("{:?}",buf);
        // do some bitshift magic to convert from rust DateTime format to the RTC's format (idk why they designed it like this).
        return Ok(DateTime{
            year:(((buf[0x06] & 0b00001111) + ((buf[0x06] >> 4)*10)) as u16) + ((((buf[0x05] >> 7) as u16)*1000u16) +1000u16),
            month: (buf[0x05] & 0b00001111) + ((buf[0x05] & 0b00010000) >> 4) * 10,
            day: ((buf[0x04] & 0b00001111) + (buf[0x04] >> 4)*10),
            day_of_week: match day_of_week_from_u8(buf[0x03]-1) {Ok(day) => day, Err(_) => DayOfWeek::Monday}, // find the day of the week, if unsure, just pick monday
            hour: ((buf[0x02] & 0b00001111) + ((buf[0x02] & 0b00110000) >> 4)*10),
            minute: ((buf[0x01] & 0b00001111) + (buf[0x01] >> 4)*10),
            second: ((buf[0x00] & 0b00001111) + (buf[0x00] >> 4)*10),
        });
    }

    pub async fn set_time(&mut self, dt:DateTime) -> Result<(),<I2C as ErrorType>::Error>{
        // unpack the datetime
        //[136, 0, 24, 128, 37, 69, 21, 2, 18, 7, 35, 0, 0, 0, 0, 0, 0, 0, 76]
        //datetime:DateTime { year: 1023, month: 7, day: 12, day_of_week: Monday, hour: 15, minute: 45, secon: 25 }

        let DateTime{year, month, day, day_of_week, hour, minute, second} = dt;
        // numbers are for proper configurations
        let month = month & 0b01111111;
        let year_tens = year - (((year>1999) as u16) * 1000 + 1000);
        let buf =[
            0, // index to start at
            ((second/10)<<4) | (second-((second/10)*10)), 
            ((minute/10)<<4) | (minute-((minute/10)*10)),
            0b01000000 | ((hour/10)<<4) | (hour-((hour/10)*10)),
            day_of_week as u8 + 1,
            day,
            (((year > 1999) as u8) << 7) | ((month/10)<<4)| (month-((month/10)*10)),
            ((year_tens/10)<<4) as u8 | (year_tens-((year_tens/10)*10)) as u8];
        self.ic2_bus.write(RTC_DEFAULT_ADDR, &buf).await?;
        Ok(())
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