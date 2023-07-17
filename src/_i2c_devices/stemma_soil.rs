use embedded_hal_async::i2c::I2c;

use super::I2cDevice;
use core::marker::PhantomData;




pub struct StemmaSoil<I2C:I2c>{
    address: u8,
    is_connected: bool,
    _i2c: PhantomData<I2C>
}

impl<I2C:I2c> I2cDevice for StemmaSoil<I2C>{
    fn get_address(&self) -> u8 {
        
    }

    fn is_connected(&self) -> bool {
        todo!()
    }

    fn get_default_address(&self) -> u8 {
        todo!()
    }
}