#![no_std]
#![no_main]

use buzzer_song::{
    music::{self, Song},
    pink_panther,
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        HighSpeed, Ledc,
    },
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut buzzer = peripherals.GPIO33;

    let ledc = Ledc::new(peripherals.LEDC);

    let song = Song::new(pink_panther::TEMPO);

    let delay = Delay::new();

    for (note, duration_type) in pink_panther::MELODY {
        let note_duration = song.calc_note_duration(duration_type);
        let pause_duration = note_duration / 10; // 10% of note_duration
        if note == music::REST {
            delay.delay_millis(note_duration);
            continue;
        }
        let freq = (note as u32).Hz();

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

        delay.delay_millis(note_duration - pause_duration); // play 90%
        channel0.set_duty(0).unwrap();
        delay.delay_millis(pause_duration); // Pause for 10%
    }

    loop {
        delay.delay_millis(5);
    }
}
