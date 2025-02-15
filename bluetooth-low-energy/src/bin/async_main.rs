#![no_std]
#![no_main]

use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    async_attribute_server::AttributeServer,
    asynch::Ble,
    gatt,
};
use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{prelude::*, rng::Rng, time, timer::timg::TimerGroup};
use esp_println::println;
use esp_wifi::{ble::controller::BleConnector, init, EspWifiController};

extern crate alloc;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init = &*mk_static!(
        EspWifiController<'static>,
        init(
            timg0.timer0,
            Rng::new(peripherals.RNG),
            peripherals.RADIO_CLK,
        )
        .unwrap()
    );

    let timg1 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timg1.timer0);

    let mut bluetooth = peripherals.BT;

    let connector = BleConnector::new(init, &mut bluetooth);

    let now = || time::now().duration_since_epoch().to_millis();
    let mut ble = Ble::new(connector, now);
    println!("Connector created");

    println!("{:?}", ble.init().await);
    println!("{:?}", ble.cmd_set_le_advertising_parameters().await);
    println!(
        "{:?}",
        ble.cmd_set_le_advertising_data(
            create_advertising_data(&[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::CompleteLocalName("implRust"),
                // AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
            ])
            .unwrap()
        )
        .await
    );
    println!("{:?}", ble.cmd_set_le_advertise_enable(true).await);

    println!("started advertising");

    let sensor_data = b"Hello, Ferris";

    let mut read_func = |_offset: usize, data: &mut [u8]| {
        data[0..sensor_data.len()].copy_from_slice(&sensor_data[..]);
        sensor_data.len()
    };
    let mut write_func = |offset: usize, data: &[u8]| {
        println!("RECEIVED: {} {:?}", offset, data);
    };

    let mut write_func2 = |offset: usize, data: &[u8]| {
        println!("RECEIVED: {} {:?}", offset, data);
    };

    gatt!([service {
        uuid: "a9c81b72-0f7a-4c59-b0a8-425e3bcf0a0e",
        characteristics: [
            characteristic {
                uuid: "13c0ef83-09bd-4767-97cb-ee46224ae6db",
                read: read_func,
                write: write_func,
            },
            characteristic {
                uuid: "c79b2ca7-f39d-4060-8168-816fa26737b7",
                write: write_func2,
            },
        ],
    },]);

    let mut rng = bleps::no_rng::NoRng;
    let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut rng);

    while srv.do_work().await.is_ok() {}
}
