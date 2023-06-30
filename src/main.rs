#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{gpio,peripherals::{PIN_0,PIN_1}};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blink1(mut led:Output<'static,PIN_0>) -> !{
    loop{
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

    }
}

#[embassy_executor::task]
async fn blink2(mut led:Output<'static,PIN_1>) -> !{
    loop{
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_0, Level::Low);

    let led2 = Output::new(p.PIN_1, Level::Low);

    let thing = blink1(led);
    let thing2 = blink2(led2);


    spawner.spawn(thing).unwrap();
    spawner.spawn(thing2).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

}