use esp_hal::time::Duration;
use esp_hal::time::Instant;

pub trait Task: Sync {
    fn run(&self);
}

#[derive(Clone, Copy)]
pub enum TaskState {
    Ready,
    Running,
    Delayed(Duration, Instant),
    _Blocked(TaskContext),
    _Terminated,
}

#[derive(Clone, Copy)]
pub struct TaskContext {}
