use core::cell::RefCell;
use critical_section::Mutex;

use crate::task::*;

pub struct Scheduler {
    tasks: &'static [&'static dyn Task],
}

static SCHEDULER: Mutex<RefCell<Option<Scheduler>>> = Mutex::new(RefCell::new(None));

impl Scheduler {
    pub fn init(tasks: &'static [&'static dyn Task]) {
        critical_section::with(|cs| {
            *SCHEDULER.borrow(cs).borrow_mut() = Some(Scheduler { tasks });
        });
    }

    pub fn run() -> ! {
        loop {
            critical_section::with(|cs| {
                if let Some(ref mut scheduler) = *SCHEDULER.borrow(cs).borrow_mut() {
                    for task in scheduler.tasks.iter() {
                        task.run();
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
