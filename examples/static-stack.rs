#![no_std]
#![no_main]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use panic_rtt_target as _;
use rustos::task::Run;

esp_bootloader_esp_idf::esp_app_desc!();

const STACK_SIZE: usize = 0x10000;

const NUMBER: u32 = 123;

struct Task {
    number: u32,
}

impl Run for Task {
    fn run(&mut self) {
        let delay = esp_hal::delay::Delay::new();

        let worker_var = 0u32;
        let worker_sp = core::ptr::addr_of!(worker_var) as usize;

        loop {
            self.number += 1;
            info!("Success! Worker stack address: {:x}", worker_sp);
            info!("Received number: {}", self.number);
            delay.delay_millis(2000);
        }
    }

    fn exit(&mut self) {
        info!("Number is now: {}", self.number);
        panic!("Task has exited");
    }
}

#[repr(C, align(16))]
struct Stack<const N: usize>([u8; N]);

#[unsafe(link_section = ".bss")]
static TASK_STACK: Stack<STACK_SIZE> = Stack([0u8; STACK_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn stack_switch_trampoline(task: &Task) -> ! {
    core::arch::naked_asm!(
        "
        la t0, {stack_addr}
        
        li t1, {stack_size}
        la ra, {exit_func}
        add t0, t0, t1
        
        mv sp, t0
        
        j {target_func}
        ",
        stack_addr = sym TASK_STACK,
        stack_size = const STACK_SIZE,
        target_func = sym <Task as Run>::run,
        exit_func = sym <Task as Run>::exit,
    );
}

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let main_var = 0u32;
    info!(
        "Main stack address: {:x}",
        core::ptr::addr_of!(main_var) as usize
    );

    let task = Task { number: NUMBER };

    info!("Switching to static stack...");

    unsafe {
        stack_switch_trampoline(&task);
    }
}

fn my_worker_thread(number: u32) {
    let delay = esp_hal::delay::Delay::new();

    let worker_var = 0u32;
    let worker_sp = core::ptr::addr_of!(worker_var) as usize;

    loop {
        info!("Success! Worker stack address: {:x}", worker_sp);
        info!("Received number: {}", number);
        delay.delay_millis(2000);
    }
}
