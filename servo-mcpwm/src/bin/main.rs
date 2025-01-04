#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    let clock_cfg = PeripheralClockConfig::with_frequency(1.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(peripherals.GPIO33, PwmPinConfig::UP_ACTIVE_HIGH);

    // start timer with timestamp values in the range of 0..=19999 and a frequency
    // of 50 Hz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, 50.Hz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);

    loop {
        // 0 degree (2.5% of 20_000 => 500)
        pwm_pin.set_timestamp(500);
        delay.delay(1500.millis());

        // 90 degree (7.5% of 20_000 => 1500)
        pwm_pin.set_timestamp(1500);
        delay.delay(1500.millis());

        // 180 degree (12.5% of 20_000 => 2500)
        pwm_pin.set_timestamp(2500);
        delay.delay(1500.millis());
    }
}
