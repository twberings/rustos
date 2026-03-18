#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::main;
use esp_hal::{clock::CpuClock, delay::Delay};

use defmt::info;

use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use panic_rtt_target as _;
use rustos::prelude::*;
use smart_leds::hsv::{Hsv, hsv2rgb};
use smart_leds::{RGB8, SmartLedsWrite as _, brightness, gamma};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);

    let mut _scheduler = Scheduler::new();

    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &mut rmt_buffer);
    let delay = Delay::new();

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data: RGB8;
    let level = 20;

    loop {
        for hue in 0..=255 {
            color.hue = hue;
            data = hsv2rgb(color);
            led.write(brightness(gamma([data].into_iter()), level))
                .unwrap();
            delay.delay_millis(1);
        }
    }

    // scheduler
    //     .spawn_task(1, blink_task)
    //     .expect("Failed to spawn blink task");
    // scheduler.run();
}

fn _blink_task(task: &mut Task) {
    info!("Blink!");
    task.delay(1000);
}
