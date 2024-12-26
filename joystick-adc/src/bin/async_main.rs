#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::peripherals::ADC1;
use esp_hal::prelude::*;
use log::info;

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    let mut adc1_config: AdcConfig<ADC1> = AdcConfig::new();
    let mut adc_pin = adc1_config.enable_pin(peripherals.GPIO34, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    loop {
        info!("Hello world!");
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut adc_pin)).unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
