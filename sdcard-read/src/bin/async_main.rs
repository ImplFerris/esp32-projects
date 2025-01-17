#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
    spi::{
        master::{Config, Spi},
        SpiMode,
    },
};
use esp_println::{print, println};
use log::info;

#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

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

    let delay = Delay::new();

    let spi = Spi::new_with_config(
        peripherals.SPI2,
        Config {
            frequency: 400.kHz(),
            mode: SpiMode::Mode0,
            ..Config::default()
        },
    )
    .with_sck(peripherals.GPIO14)
    .with_mosi(peripherals.GPIO15)
    .with_miso(peripherals.GPIO2);
    let sd_cs = Output::new(peripherals.GPIO13, Level::High);
    let spi = ExclusiveDevice::new(spi, sd_cs, delay).unwrap();

    let sdcard = SdCard::new(spi, delay);
    let mut volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());
    loop {
        // Timer::after(Duration::from_secs(10)).await;
        println!("Init SD card controller and retrieve card size...");
        match volume_mgr.device().num_bytes() {
            Ok(size) => {
                println!("card size is {} bytes\r\n", size);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        }

        let mut volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

        let mut root_dir = volume0.open_root_dir().unwrap();

        let mut my_file = root_dir
            .open_file_in_dir("RUST.TXT", embedded_sdmmc::Mode::ReadOnly)
            .unwrap();

        while !my_file.is_eof() {
            let mut buffer = [0u8; 32];

            if let Ok(n) = my_file.read(&mut buffer) {
                for b in &buffer[..n] {
                    print!("{}", *b as char);
                }
            }
        }

        Timer::after(Duration::from_secs(15)).await;
    }
}
