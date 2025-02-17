#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, Pull};
use esp_hal::prelude::*;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let sensor_pin = Input::new(peripherals.GPIO33, Pull::Down);

    let delay = Delay::new();
    loop {
        if sensor_pin.is_high() {
            println!("Motion detected");
            delay.delay(100.millis());
        }
        delay.delay(100.millis());
    }
}
