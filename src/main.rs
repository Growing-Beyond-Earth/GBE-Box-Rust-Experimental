#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use embassy_executor::Spawner;
use embassy_rp::{Peripherals,pwm::{Pwm,Config}};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use {defmt_rtt as _, panic_probe as _};
use once_cell::sync::Lazy;

mod blinky;
mod wifi;

// this variable holds the peripheral access with lazy initialization. Memory is allocated at compile time but the value is set at runtime
static PERIPHERALS:Lazy<Peripherals> = Lazy::new(|| {
    embassy_rp::init(Default::default())
});


// The video below explains pico PWM very well. compare_a and compare_b are the set points (for channel a and b)
// and top is the wrap point for the PWM channel.
// https://www.youtube.com/watch?v=Au-oc4hxj-c
#[embassy_executor::task]
async fn startup_led() -> (){
    let mut pwm_config: Config = Default::default();
    pwm_config.top = 10000;
    pwm_config.divider = 250.to_fixed();
    pwm_config.compare_a = 0;
    
    let mut neopixel = Pwm::new_output_a(&PERIPHERALS.PWM_CH0, &PERIPHERALS.PIN_0, pwm_config.clone());
    loop{
        Timer::after(Duration::from_secs(1)).await;
        for i in 175..1150 {
            Timer::after(Duration::from_millis(1)).await;
            pwm_config.compare_a = i;
            neopixel.set_config(&pwm_config);
        }
        Timer::after(Duration::from_secs(1)).await;
        for i in (175..1150).rev() {
            Timer::after(Duration::from_millis(1)).await;
            pwm_config.compare_a = i;
            neopixel.set_config(&pwm_config);
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(blinky::blinky(&PERIPHERALS)).unwrap();
    spawner.spawn(startup_led()).unwrap();
}