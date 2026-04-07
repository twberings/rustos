use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use esp_hal::interrupt::{self, software::SoftwareInterrupt};

use crate::task::*;

pub struct Scheduler {
    tasks: TaskList,
}

static SCHEDULER: Mutex<RefCell<Option<Scheduler>>> = Mutex::new(RefCell::new(None));

impl Scheduler {
    pub fn init(tasks: TaskList) {
        critical_section::with(|cs| {
            *SCHEDULER.borrow(cs).borrow_mut() = Some(Scheduler { tasks });
        });

        let interrupt = esp_hal::peripherals::Interrupt::FROM_CPU_INTR0;

        interrupt::enable_direct(
            interrupt,
            interrupt::Priority::min(),
            interrupt::CpuInterrupt::Interrupt22,
            handler,
        )
        .expect("Failed to enable interrupt");
    }

    pub fn run() -> ! {
        loop {
            unsafe {
                // SoftwareInterrupt::<'static, 0>::steal().raise();
                core::arch::asm!("wfi");
            }

            critical_section::with(|cs| {
                if let Some(ref mut scheduler) = *SCHEDULER.borrow(cs).borrow_mut() {
                    for task in scheduler.tasks.tasks.iter() {
                        if let TaskState::Ready = task.state {
                            task.run();
                        }
                    }
                }
            })
        }
    }
}

#[derive(Debug)]
pub enum SchedulerError {
    NoAvailableSlot,
    SchedulerNotInitialized,
}

#[unsafe(link_section = ".trap.rust")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[rustfmt::skip]
unsafe extern "C" fn handler() {
    core::arch::naked_asm!("
        .cfi_startproc
        la t0, {interrupt_handler}
        jalr ra, t0, 0
        mret
        .cfi_endproc
", interrupt_handler = sym interrupt_handler);
}

#[esp_hal::ram]
extern "C" fn interrupt_handler() {
    info!("Interrupt received, running scheduler");
    unsafe {
        SoftwareInterrupt::<'static, 0>::steal().reset();
    }
}
