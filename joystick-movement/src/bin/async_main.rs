#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    gpio::{Input, Pull},
    prelude::*,
};
use esp_println::println;
use log::info;

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

    let btn = Input::new(peripherals.GPIO32, Pull::Up);

    let mut adc2_config = AdcConfig::new();
    let mut vrx_pin = adc2_config.enable_pin(peripherals.GPIO13, Attenuation::Attenuation11dB);
    let mut vry_pin = adc2_config.enable_pin(peripherals.GPIO14, Attenuation::Attenuation11dB);

    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);

    // let delay = Delay::new();

    let mut prev_vrx: u16 = 0;
    let mut prev_vry: u16 = 0;
    let mut prev_btn_state = false;
    let mut print_vals = true;

    loop {
        let Ok(vry): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vry_pin)) else {
            continue;
        };
        let Ok(vrx): Result<u16, _> = nb::block!(adc2.read_oneshot(&mut vrx_pin)) else {
            continue;
        };

        if vrx.abs_diff(prev_vrx) > 100 {
            prev_vrx = vrx;
            print_vals = true;
        }

        if vry.abs_diff(prev_vry) > 100 {
            prev_vry = vry;
            print_vals = true;
        }

        let btn_state = btn.is_low();
        if btn_state && !prev_btn_state {
            println!("Button Pressed");
            print_vals = true;
        }
        prev_btn_state = btn_state;

        if print_vals {
            print_vals = false;

            println!("X: {} Y: {}\r\n", vrx, vry);
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}
