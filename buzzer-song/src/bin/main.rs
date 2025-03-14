#![no_std]
#![no_main]

use buzzer_song::{
    music::{self, Song},
    pink_panther,
};

use esp_hal::{
    clock::CpuClock,
    ledc::{
        channel::{self, ChannelIFace},
        timer::TimerIFace,
        Ledc,
    },
    time::Rate,
};
use esp_hal::{ledc::timer, main};
use esp_hal::{
    ledc::HighSpeed,
    time::{Duration, Instant},
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut buzzer = peripherals.GPIO33;

    let ledc = Ledc::new(peripherals.LEDC);

    let song = Song::new(pink_panther::TEMPO);

    for (note, duration_type) in pink_panther::MELODY {
        let note_duration = song.calc_note_duration(duration_type) as u64;
        let pause_duration = note_duration / 10; // 10% of note_duration
        if note == music::REST {
            blocking_delay(Duration::from_millis(note_duration));
            continue;
        }
        let freq = Rate::from_hz(note as u32);

        let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
        hstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty10Bit,
                clock_source: timer::HSClockSource::APBClk,
                frequency: freq,
            })
            .unwrap();

        let mut channel0 = ledc.channel(channel::Number::Channel0, &mut buzzer);
        channel0
            .configure(channel::config::Config {
                timer: &hstimer0,
                duty_pct: 50,
                pin_config: channel::config::PinConfig::PushPull,
            })
            .unwrap();

        blocking_delay(Duration::from_millis(note_duration - pause_duration)); // play 90%

        channel0.set_duty(0).unwrap();
        blocking_delay(Duration::from_millis(pause_duration)); // Pause for 10%
    }

    loop {
        blocking_delay(Duration::from_millis(5));
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
