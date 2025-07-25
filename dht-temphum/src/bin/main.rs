#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::{error, info};
use dht22_sensor::{Dht22, DhtError};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{self, Flex, Level};
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.4.0
    // TODO: Spawn some tasks
    let _ = spawner;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let delay = Delay::new();
    let mut dht_pin = Flex::new(peripherals.GPIO4);
    let output_config = gpio::OutputConfig::default()
        .with_drive_mode(gpio::DriveMode::OpenDrain)
        .with_pull(gpio::Pull::None);
    dht_pin.apply_output_config(&output_config);
    dht_pin.set_input_enable(true);
    dht_pin.set_output_enable(true);
    dht_pin.set_level(Level::High);

    let mut sensor = Dht22::new(dht_pin, delay);

    Timer::after(Duration::from_secs(2)).await;

    loop {
        match sensor.read() {
            Ok(reading) => info!(
                "Temperature: {}, Humidity: {}",
                reading.temperature, reading.relative_humidity
            ),
            Err(err) => match err {
                DhtError::ChecksumMismatch => {
                    error!("checksum error");
                }
                DhtError::Timeout => {
                    error!("Timeout error");
                }
                DhtError::PinError(e) => {
                    error!("Pin error:{}", e);
                }
            },
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}
