use esp_hal::delay::Delay;

#[derive(Clone, Copy)]
pub struct Task {
    _priority: usize,
    entry: fn(&mut Task),
    pub state: TaskState,
    d: Delay,
}

#[derive(Clone, Copy)]
pub enum TaskState {
    Ready,
    Running,
    _Blocked(TaskContext),
    _Terminated,
}

#[derive(Clone, Copy)]
pub struct TaskContext {}

impl Task {
    pub fn new(priority: usize, entry: fn(&mut Task)) -> Self {
        Task {
            _priority: priority,
            entry,
            state: TaskState::Ready,
            d: Delay::new(),
        }
    }
    pub fn delay(&mut self, millis: u32) {
        self.d.delay_millis(millis);
    }
    pub fn run(&mut self) {
        self.state = TaskState::Running;
        (self.entry)(self);
        // self.state = TaskState::Ready;
    }
}
