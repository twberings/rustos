#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::sync::atomic::AtomicUsize;
use esp_hal::main;

use esp_hal::clock::CpuClock;

use defmt::info;

use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[repr(C)]
#[derive(Default)]
pub struct TaskContext {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
    pub pc: usize,
}

pub struct Task {
    pub sp: AtomicUsize,
}

#[repr(align(16))]
struct Stack<const SIZE: usize> {
    data: [u8; SIZE],
}

#[unsafe(link_section = ".bss")]
static mut TASK_STACK: Stack<4096> = Stack { data: [0u8; 4096] };

// The current task pointer for the assembly to find
#[unsafe(no_mangle)]
static mut CURRENT_TASK_PTR: usize = 0;

pub fn init_first_task(task: &Task, entry: extern "C" fn()) {
    unsafe {
        let stack_bottom = core::ptr::addr_of_mut!(TASK_STACK.data) as usize;
        let stack_top = stack_bottom + 4096;

        // Reserve space for context
        let context_ptr = (stack_top - core::mem::size_of::<TaskContext>()) as *mut TaskContext;

        let ctx = TaskContext {
            pc: entry as usize,       // Start execution at the entry point
            sp: context_ptr as usize, // SP starts at the context struct
            ..TaskContext::default()
        };

        core::ptr::write(context_ptr, ctx);
        task.sp
            .store(context_ptr as usize, core::sync::atomic::Ordering::SeqCst);

        // Point the global pointer to this task
        CURRENT_TASK_PTR = &task.sp as *const _ as usize;
    }
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn load_task_interrupt() {
    core::arch::naked_asm!(
        "
        # 1. Load the pointer to the saved context
        la t0, CURRENT_TASK_PTR
        lw t0, 0(t0)        # t0 = &MY_TASK.sp
        lw t0, 0(t0)        # t0 = &MY_TASK.sp

        mv sp, t0           # Set the stack pointer to the task's context

        # 2. Restore all General Purpose Registers
        lw ra, 0*4(sp)
        lw t0, 1*4(sp)
        lw t1, 2*4(sp)
        lw t2, 3*4(sp)
        lw t3, 4*4(sp)
        lw t4, 5*4(sp)
        lw t5, 6*4(sp)
        lw t6, 7*4(sp)
        lw a0, 8*4(sp)
        lw a1, 9*4(sp)
        lw a2, 10*4(sp)
        lw a3, 11*4(sp)
        lw a4, 12*4(sp)
        lw a5, 13*4(sp)
        lw a6, 14*4(sp)
        lw a7, 15*4(sp)
        lw s0, 16*4(sp)
        lw s1, 17*4(sp)
        lw s2, 18*4(sp)
        lw s3, 19*4(sp)
        lw s4, 20*4(sp)
        lw s5, 21*4(sp)
        lw s6, 22*4(sp)
        lw s7, 23*4(sp)
        lw s8, 24*4(sp)
        lw s9, 25*4(sp)
        lw s10, 26*4(sp)
        lw s11, 27*4(sp)
        lw gp, 28*4(sp)
        lw tp, 29*4(sp)

        # 3. Load the Target PC into MEPC
        lw t1, 31*4(sp)
        csrw mepc, t1

        # 4. Pop the TaskContext from the stack before jumping
        addi sp, sp, 128    # 32 registers * 4 bytes = 128

        mret
        "
    );
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    info!("Starting main task!");

    static MY_TASK: Task = Task {
        sp: AtomicUsize::new(0),
    };

    init_first_task(&MY_TASK, my_worker_thread);

    unsafe {
        core::arch::asm!("j load_task_interrupt");
    }

    panic!();
}

extern "C" fn my_worker_thread() {
    loop {
        esp_hal::delay::Delay::new().delay_millis(1000);
        info!("Worker thread is running!");
    }
}
