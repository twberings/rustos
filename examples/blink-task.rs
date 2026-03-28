#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;

use critical_section::Mutex;
use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use panic_rtt_target as _;
use rustos::prelude::*;

esp_bootloader_esp_idf::esp_app_desc!();

struct BlinkTask {
    delay: Delay,
    number: Mutex<RefCell<u32>>,
}

impl Task for BlinkTask {
    fn run(&self) {
        self.delay.delay_millis(1000);
        critical_section::with(|cs| {
            let mut number = self.number.borrow(cs).borrow_mut();
            *number += 1;
            info!("Blink number: {}", *number);
        });
    }
}

static BLINK_TASK: BlinkTask = BlinkTask {
    delay: Delay::new(),
    number: Mutex::new(RefCell::new(0)),
};

static TASKS: &[&dyn Task] = &[&BLINK_TASK];

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    Scheduler::init(TASKS);
    Scheduler::run();
}
