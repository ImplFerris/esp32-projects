#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
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

    let mut led = Output::new(peripherals.GPIO33, Level::Low);

    let adc_pin = peripherals.GPIO4;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(adc_pin, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC2, adc1_config);
    let delay = Delay::new();

    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();
        esp_println::println!("{}", pin_value);

        if pin_value > 3500 {
            led.set_high();
        } else {
            led.set_low();
        }

        delay.delay_millis(500);
    }
}
