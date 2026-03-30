use core::cell::RefCell;
use critical_section::Mutex;

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
    }

    pub fn run() -> ! {
        loop {
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
