#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let sensor_pin = Input::new(
        peripherals.GPIO33,
        InputConfig::default().with_pull(Pull::Down),
    );

    let mut buzzer_pin = Output::new(peripherals.GPIO18, Level::Low, OutputConfig::default());
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    loop {
        if sensor_pin.is_high() {
            buzzer_pin.set_high();
            led.set_high();
            blocking_delay(Duration::from_millis(100));
            buzzer_pin.set_low();
            led.set_low();
        }
        blocking_delay(Duration::from_millis(100));
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
