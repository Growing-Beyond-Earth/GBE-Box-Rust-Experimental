#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use embassy_executor::Spawner;
use embassy_rp::{gpio,Peripherals};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use once_cell::sync::Lazy;

static P:Lazy<Peripherals> = Lazy::new(|| {
    embassy_rp::init(Default::default())
});


#[embassy_executor::task]
async fn blink1() -> !{
    let mut led = Output::new(&P.PIN_0, Level::Low);
    loop{
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}


#[embassy_executor::task]
async fn blink2() -> !{
    let mut led = Output::new(&P.PIN_1, Level::Low);
    loop{
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(blink1()).unwrap();
    spawner.spawn(blink2()).unwrap();
}