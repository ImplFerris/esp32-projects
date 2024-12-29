#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut buzzer = Output::new(peripherals.GPIO33, Level::Low);

    let delay = Delay::new();
    loop {
        buzzer.set_high();
        delay.delay_millis(500);
        buzzer.set_low();
        delay.delay_millis(500);
    }
}
