use embassy_rp::{gpio,Peripherals};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};


// blinks the status led
#[embassy_executor::task]
pub async fn blinky(p: &'static Peripherals) -> ! {
    let mut led = Output::new(&p.PIN_25, Level::Low);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(250)).await;
        led.set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}
