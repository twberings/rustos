use crate::task::*;
const MAX_TASKS: usize = 10;

#[derive(Default)]
pub struct Scheduler {
    tasks: [Option<Task>; MAX_TASKS],
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: [None; MAX_TASKS],
        }
    }

    pub fn spawn_task(
        &mut self,
        priority: usize,
        entry: fn(&mut Task),
    ) -> Result<&Task, SchedulerError> {
        for slot in self.tasks.iter_mut() {
            if slot.is_none() {
                *slot = Some(Task::new(priority, entry));
                return Ok(slot.as_ref().unwrap());
            }
        }
        Err(SchedulerError::NoAvailableSlot)
    }

    pub fn run(&mut self) -> ! {
        loop {
            for task in self.tasks.iter_mut().flatten() {
                match task.state {
                    TaskState::Ready => task.run(),
                    TaskState::Running => continue,
                    _ => continue,
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum SchedulerError {
    NoAvailableSlot,
}
