use embassy_executor;
use embedded_hal_async::i2c::I2c;
use embedded_hal_1::i2c::ErrorType;
use super::ds3231;


#[embassy_executor::task]
async fn periodically_print(rtc:&ds3231::Ds3231<'_,I2c>){

}