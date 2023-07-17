use embassy_rp::rtc::{DateTime, DayOfWeek, DateTimeError};
use embassy_sync::{mutex::Mutex, blocking_mutex::raw::{RawMutex, ThreadModeRawMutex}};
use embedded_hal_1::i2c::ErrorType;
use embedded_hal_async::i2c::I2c;
use log::info;

const RTC_DEFAULT_ADDR:u8 = 0x68;

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

pub struct Ds3231<'a,I2C:I2c>{
    i2c_mutex: &'a Mutex<ThreadModeRawMutex,I2C>
}

impl<'a, I2C> Ds3231<'a,I2C>
where
    I2C: I2c,
    {
    pub fn new(i2c_bus:&'a Mutex<ThreadModeRawMutex, I2C>) -> Self{
        Ds3231{i2c_mutex: i2c_bus}
    }

    pub async fn get_rtc_datetime(&self) -> Result<DateTime,<I2C as ErrorType>::Error>{
        let mut i2c_bus = self.i2c_mutex.lock().await;
        let mut buf:[u8;7] = [0;7];
        i2c_bus.write_read(RTC_DEFAULT_ADDR,&[0], &mut buf).await?;
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

    pub async fn set_rtc_datetime(&self, dt:DateTime) -> Result<(),<I2C as ErrorType>::Error>{
        let mut i2c_bus = self.i2c_mutex.lock().await;
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
        i2c_bus.write(RTC_DEFAULT_ADDR, &buf).await?;
        Ok(())
    }
}