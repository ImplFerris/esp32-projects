#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, Level, Output, Pull};
use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let sensor_pin = Input::new(peripherals.GPIO33, Pull::Down);

    let mut buzzer_pin = Output::new(peripherals.GPIO18, Level::Low);
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    let delay = Delay::new();
    loop {
        if sensor_pin.is_high() {
            buzzer_pin.set_high();
            led.set_high();
            delay.delay(100.millis());
            buzzer_pin.set_low();
            led.set_low();
        }
        delay.delay(100.millis());
    }
}
