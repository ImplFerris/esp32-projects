#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use dht22_sensor::{Dht22, DhtError};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{self, Flex, Level};
use esp_hal::main;
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut dht_pin = Flex::new(peripherals.GPIO4);

    let output_config = gpio::OutputConfig::default()
        .with_drive_mode(gpio::DriveMode::OpenDrain)
        .with_pull(gpio::Pull::None);
    dht_pin.apply_output_config(&output_config);
    dht_pin.set_input_enable(true);
    dht_pin.set_output_enable(true);
    dht_pin.set_level(Level::High);

    let mut delay = Delay::new();
    let delay1 = Delay::new();
    delay1.delay_millis(2000);

    let mut sensor = Dht22::new(&mut dht_pin, &mut delay);
    loop {
        match sensor.read() {
            Ok(reading) => {
                info!(
                    "Temperature: {:?}, Humidity: {:?}",
                    reading.temperature, reading.relative_humidity
                );
            }
            Err(err) => match err {
                DhtError::ChecksumMismatch => {
                    info!("checksum error");
                }
                DhtError::Timeout => {
                    info!("Timeout error");
                }
                DhtError::PinError(e) => {
                    info!("Pin error:{}", e);
                }
            },
        }
        delay1.delay_millis(5000);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
