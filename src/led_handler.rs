

mod ws2812;
use ws2812::Ws2812;

use embassy_rp::{pio::Pio, peripherals::{PIO1, PIN_6, DMA_CH1}};
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;


#[embassy_executor::task]
pub async fn startup_led(led_pio:PIO1, led_pin:PIN_6, led_dma:DMA_CH1) -> () {
    let Pio { mut common, sm0, .. } = Pio::new(led_pio);
    let mut ws2812:Ws2812<'_, embassy_rp::peripherals::PIO1, 0, 1> = Ws2812::new(&mut common, sm0, led_dma, led_pin);
    loop{
    for i in 0..=255{
        let data = [RGB8::new(0, i , 0);1];
        ws2812.write(&data).await;
        Timer::after(Duration::from_millis(7)).await;
    }
    for i in (0..=255).rev(){
        let data = [RGB8::new(0, i , 0);1];
        ws2812.write(&data).await;
        Timer::after(Duration::from_millis(7)).await;
    }
}
}


// use {defmt_rtt as _, panic_probe as _};



// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]



// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     info!("Start");
//     let p = embassy_rp::init(Default::default());

//     let Pio { mut common, sm0, .. } = Pio::new(p.PIO0);

//     // This is the number of leds in the string. Helpfully, the sparkfun thing plus and adafruit
//     // feather boards for the 2040 both have one built in.
//     const NUM_LEDS: usize = 1;
//     let mut data = [RGB8::default(); NUM_LEDS];

//     // For the thing plus, use pin 8
//     // For the feather, use pin 16
//     let mut ws2812 = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_0);

//     // Loop forever making RGB values and pushing them out to the WS2812.
//     loop {
//         for j in 0..(256 * 5) {
//             debug!("New Colors:");
//             for i in 0..NUM_LEDS {
//                 data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
//                 debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
//             }
//             ws2812.write(&data).await;

//             Timer::after(Duration::from_micros(5)).await;
//         }
//     }
// }
