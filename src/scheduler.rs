use core::cell::RefCell;
use critical_section::Mutex;

use crate::task::*;
const MAX_TASKS: usize = 10;

#[derive(Default)]
pub struct Scheduler {
    tasks: [Option<Task>; MAX_TASKS],
}

static SCHEDULER: Mutex<RefCell<Option<Scheduler>>> = Mutex::new(RefCell::new(None));

impl Scheduler {
    pub fn init() {
        critical_section::with(|cs| {
            *SCHEDULER.borrow(cs).borrow_mut() = Some(Scheduler::default());
        });
    }

    pub fn spawn_task(priority: usize, entry: fn(&mut Task)) -> Result<(), SchedulerError> {
        critical_section::with(|cs| {
            if let Some(ref mut scheduler) = *SCHEDULER.borrow(cs).borrow_mut() {
                for slot in scheduler.tasks.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(Task::new(priority, entry));
                        return Ok(());
                    }
                }
                return Err(SchedulerError::NoAvailableSlot);
            }
            Err(SchedulerError::SchedulerNotInitialized)
        })
    }

    pub fn run() -> ! {
        loop {
            critical_section::with(|cs| {
                if let Some(ref mut scheduler) = *SCHEDULER.borrow(cs).borrow_mut() {
                    for task in scheduler.tasks.iter_mut().flatten() {
                        match task.state {
                            TaskState::Ready => task.run(),
                            TaskState::Running => continue,
                            _ => continue,
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
