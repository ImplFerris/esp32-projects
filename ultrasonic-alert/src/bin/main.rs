#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pull},
    prelude::*,
    rtc_cntl::Rtc,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut buzzer = Output::new(peripherals.GPIO33, Level::Low);

    // For HC-SR04 Ultrasonic
    let mut trig = Output::new(peripherals.GPIO5, Level::Low);
    let echo = Input::new(peripherals.GPIO18, Pull::Down);

    let delay = Delay::new();
    let rtc = Rtc::new(peripherals.LPWR);

    loop {
        delay.delay_millis(5);

        // Trigger ultrasonic waves
        trig.set_low();
        delay.delay_micros(2);
        trig.set_high();
        delay.delay_micros(10);
        trig.set_low();

        // Measure the duration the signal remains high
        while echo.is_low() {}
        let time1 = rtc.current_time();
        while echo.is_high() {}
        let time2 = rtc.current_time();
        let pulse_width = match (time2 - time1).num_microseconds() {
            Some(pw) => pw as f64,
            None => continue,
        };

        // Derive distance from the pulse width
        let distance = (pulse_width * 0.0343) / 2.0;
        // esp_println::println!("Pulse Width: {}", pulse_width);
        // esp_println::println!("Distance: {}", distance);

        if distance < 30.0 {
            buzzer.set_high();
        } else {
            buzzer.set_low();
        }

        delay.delay_millis(60);
    }
}
