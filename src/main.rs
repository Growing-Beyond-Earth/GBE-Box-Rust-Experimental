#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};


mod led_handler;

// blinks the status led

// this variable holds the peripheral access with lazy initialization. Memory is allocated at compile time but the value is set at runtime



// The video below explains pico PWM very well. compare_a and compare_b are the set points (for channel a and b)
// and top is the wrap point for the PWM channel.
// https://www.youtube.com/watch?v=Au-oc4hxj-c

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(led_handler::startup_led()).unwrap();
}

