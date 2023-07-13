#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_rp::rtc::DateTime;
use embassy_rp::{bind_interrupts, i2c};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0, USB, I2C0};
use embassy_rp::pio::Pio;
use embassy_time::{Duration, Timer};
use log::info;
use static_cell::make_static;
use embassy_rp::usb;
use {defmt_rtt as _, panic_probe as _};

mod led_handler;
mod ds3231;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

//loop that runs usb logger
#[embassy_executor::task]
async fn logger_task(driver: usb::Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// loop that runs wifi
#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static, PIN_23>, PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {

    //initialize peripherals
    let p = embassy_rp::init(Default::default());


    // USB LOGGING INIT:
    let driver = usb::Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // load the firmware for the rpi pico W
    let fw = include_bytes!("../bin/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../bin/cyw43-firmware/43439A0_clm.bin");

    //setup the spi bus and initialize the wifi chip. WIFI INIT
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);
    let state = make_static!(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();
    //initialize the handle for wifi
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;


    // start the led indicator
    spawner.spawn(led_handler::startup_led(p.PIO1, p.PIN_6, p.DMA_CH1)).unwrap();

    // TEST STARTS HERE!!!!!!!!!!!!!!!!
    let mut i2c_bus = i2c::I2c::new_async(p.I2C0, p.PIN_17, p.PIN_16, Irqs, embassy_rp::i2c::Config::default());

    let mut i2c_rtc = ds3231::Ds3231::new(i2c_bus);

    Timer::after(Duration::from_secs(1)).await;
    match i2c_rtc.date_time().await{
        Ok(second) => info!("datetime:{:?}",second),
        Err(_rtc_error) => info!("ERROR!")
    };
    Timer::after(Duration::from_secs(1)).await;
    match i2c_rtc.date_time().await{
        Ok(second) => info!("datetime:{:?}",second),
        Err(_rtc_error) => info!("ERROR!")
    };
    Timer::after(Duration::from_secs(5)).await;
    let funnytime = DateTime{
        year:2023,
        month:7,
        day:13,
        day_of_week:embassy_rp::rtc::DayOfWeek::Thursday,
        hour:10,
        minute:21,
        second:00,
    };
    match i2c_rtc.set_time(funnytime).await{
        Ok(second) => info!("datetime:{:?}",second),
        Err(_rtc_error) => info!("ERROR!")
    };

    loop{
        Timer::after(Duration::from_secs(1)).await;
        match i2c_rtc.date_time().await{
            Ok(second) => info!("datetime:{:?}",second),
            Err(_rtc_error) => info!("ERROR!")
        };
    }
        
    
}
