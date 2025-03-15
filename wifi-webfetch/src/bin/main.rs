#![no_std]
#![no_main]

use blocking_network_stack::Stack;
use defmt::info;
use embedded_io::{Read, Write};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::peripherals::Peripherals;
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{main, time};
use esp_println as _;
use esp_println::println;
use esp_wifi::wifi::WifiController;
use smoltcp::iface::{SocketSet, SocketStorage};
use smoltcp::wire::{DhcpOption, IpAddress};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    // https://github.com/esp-rs/esp-hal/blob/esp-hal-v1.0.0-beta.0/examples/src/bin/wifi_dhcp.rs

    let peripherals = init_hardware();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut rng = Rng::new(peripherals.RNG);

    let esp_wifi_ctrl = esp_wifi::init(timg0.timer0, rng.clone(), peripherals.RADIO_CLK).unwrap();
    let (mut controller, interfaces) =
        esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();
    let mut device = interfaces.sta;

    // let mut stack = setup_network_stack(device, &mut rng);
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
    let mut dhcp_socket = smoltcp::socket::dhcpv4::Socket::new();

    // we can set a hostname here (or add other DHCP options)
    dhcp_socket.set_outgoing_options(&[DhcpOption {
        kind: 12,
        data: b"implRust",
    }]);
    socket_set.add(dhcp_socket);

    let now = || time::Instant::now().duration_since_epoch().as_millis();
    let mut stack = Stack::new(
        create_interface(&mut device),
        device,
        socket_set,
        now,
        rng.random(),
    );

    configure_wifi(&mut controller);
    connect_wifi(&mut controller, &mut stack);

    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let mut socket = stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    http_loop(&mut socket)
}

pub fn create_interface(device: &mut esp_wifi::wifi::WifiDevice) -> smoltcp::iface::Interface {
    // users could create multiple instances but since they only have one WifiDevice
    // they probably can't do anything bad with that
    smoltcp::iface::Interface::new(
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        device,
        timestamp(),
    )
}

// some smoltcp boilerplate
fn timestamp() -> smoltcp::time::Instant {
    smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros() as i64,
    )
}

fn init_hardware() -> Peripherals {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);
    peripherals
}

fn configure_wifi(controller: &mut WifiController<'_>) {
    let client_config =
        esp_wifi::wifi::Configuration::Client(esp_wifi::wifi::ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
            ..Default::default()
        });

    let res = controller.set_configuration(&client_config);
    info!("wifi_set_configuration returned {:?}", res);

    controller.start().unwrap();
    info!("is wifi started: {:?}", controller.is_started());

    info!("Start Wifi Scan");
    let res: Result<(heapless::Vec<_, 10>, usize), _> = controller.scan_n();
    if let Ok((res, _count)) = res {
        for ap in res {
            info!("{:?}", ap);
        }
    }
}

fn connect_wifi(
    controller: &mut WifiController<'_>,
    stack: &mut Stack<'_, esp_wifi::wifi::WifiDevice<'_>>,
) {
    println!("{:?}", controller.capabilities());
    info!("wifi_connect {:?}", controller.connect());

    info!("Wait to get connected");
    loop {
        match controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
    info!("Connected: {:?}", controller.is_connected());

    info!("Wait for IP address");
    loop {
        stack.work();
        if stack.is_iface_up() {
            println!("IP acquired: {:?}", stack.get_ip_info());
            break;
        }
    }
}

fn http_loop(
    socket: &mut blocking_network_stack::Socket<'_, '_, esp_wifi::wifi::WifiDevice<'_>>,
) -> ! {
    info!("Starting HTTP client loop");
    let delay = Delay::new();
    loop {
        info!("Making HTTP request");
        socket.work();

        let remote_addr = IpAddress::v4(142, 250, 185, 115);
        socket.open(remote_addr, 80).unwrap();
        socket
            .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
            .unwrap();
        socket.flush().unwrap();

        let deadline = Instant::now() + Duration::from_secs(20);
        let mut buffer = [0u8; 512];
        while let Ok(len) = socket.read(&mut buffer) {
            // let text = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
            let Ok(text) = core::str::from_utf8(&buffer[..len]) else {
                panic!("Invalid UTF-8 sequence encountered");
            };

            info!("{}", text);

            if Instant::now() > deadline {
                info!("Timeout");
                break;
            }
        }

        socket.disconnect();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            socket.work();
        }

        delay.delay_millis(1000);
    }
}
