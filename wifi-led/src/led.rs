use core::sync::atomic::{AtomicBool, Ordering};

use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

pub static LED_STATE: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
pub async fn led_task(mut led: Output<'static>) {
    loop {
        if LED_STATE.load(Ordering::Relaxed) {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
