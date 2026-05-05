#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use defmt::info;
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[repr(C, align(16))]
struct Stack<const N: usize>([u8; N]);

#[unsafe(link_section = ".bss")]
static mut TASK_STACK: Stack<8192> = Stack([0u8; 8192]);

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn stack_switch_trampoline() {
    core::arch::naked_asm!(
        "
        la t0, {stack_addr}
        
        li t1, 0x10000
        add t0, t0, t1
        
        mv sp, t0
        
        j {target_func}
        ",
        stack_addr = sym TASK_STACK,
        target_func = sym my_worker_thread,
    );
}

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let main_var = 0u32;
    info!("Main stack address: {:x}", core::ptr::addr_of!(main_var) as usize);

    info!("Switching to static stack...");
    
    unsafe {
        stack_switch_trampoline();
    }

    panic!()
}

extern "C" fn my_worker_thread() {
    let delay = esp_hal::delay::Delay::new();
    
    let worker_var = 0u32;
    let worker_sp = core::ptr::addr_of!(worker_var) as usize;

    loop {
        info!("Success! Worker stack address: {:x}", worker_sp);
        delay.delay_millis(2000);
    }
}
