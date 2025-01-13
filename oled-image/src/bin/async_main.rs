#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
};
use esp_backtrace as _;
use esp_hal::prelude::*;
use log::info;
use ssd1306::{
    mode::DisplayConfigAsync, prelude::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};

use embedded_graphics::prelude::*;

#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    0b00111000,
    0b01000100,
    0b01000100,
    0b00101000,
    0b11101110,
];

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let i2c0 = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config {
            frequency: 400.kHz(),
            timeout: Some(100),
        },
    )
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c0);
    // initialize the display
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);

    let image = Image::new(&raw_image, Point::zero());

    image.draw(&mut display).unwrap();
    display.flush().await.unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
