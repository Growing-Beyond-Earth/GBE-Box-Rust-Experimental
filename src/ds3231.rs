use embassy_embedded_hal::shared_bus::I2cDeviceError;
use embassy_rp::rtc::DateTime;
use embassy_rp::{bind_interrupts};
use embassy_rp::i2c::{InterruptHandler};
use embassy_rp::peripherals::{I2C0};
use embedded_hal_1::i2c::ErrorType;
use embedded_hal_async::i2c::I2c;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});
enum RtcRegisters{
    SecAddr = 0x0,
    MinAddr = 0x1,
    HourAddr = 0x2,
    DayAddr = 0x3,
    DateAddr = 0x4,
    MonAddr = 0x5,
    YrAddr = 0x6,
    CtlAddr = 0x0e,
    StatAddr = 0x0f,
}

const DEFAULT_I2C_ADDR:u8 = 0x68;

pub struct Ds3231<I2C:I2c>{
    ic2_bus:I2C
}
// ds3231 datasheet https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231.pdf
impl<I2C:I2c> Ds3231<I2C>{
    pub fn new(i2c_bus:I2C) -> Self{
        Ds3231 { ic2_bus:i2c_bus }
    }

    pub async fn dateTime(&mut self) -> Result<DateTime,<I2C as ErrorType>::Error>{
        let mut buf:[u8;8] = [0;8];
        self.ic2_bus.read(DEFAULT_I2C_ADDR, &mut buf).await?;
        return DateTime{
            year:buf[0x6] as u16,
            month: buf[0x5] as u16},

    }
}