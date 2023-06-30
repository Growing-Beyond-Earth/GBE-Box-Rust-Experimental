#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::{gpio,Peripherals,pwm::{Pwm,Config}};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use once_cell::sync::Lazy;

// this variable holds the peripheral access with lazy initialization. Memory is allocated at compile time but the value is set at runtime
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
async fn startup_led() -> (){
    let mut pwm_config: Config = Default::default();
    pwm_config.top = 0x8000;
    pwm_config.compare_a = 8;
    
    let mut neopixel = Pwm::new_output_a(&P.PWM_CH0, &P.PIN_0, pwm_config.clone());
    loop{
        pwm_config.compare_a = pwm_config.compare_a.rotate_left(1);
        neopixel.set_config(&pwm_config);
        Timer::after(Duration::from_millis(1000)).await;
    }

}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // spawner.spawn(blink1()).unwrap();
    spawner.spawn(startup_led()).unwrap();
}