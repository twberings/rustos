pub trait Run: Sync {
    fn run(&self);
}

pub struct Task {
    runner: &'static dyn Run,
    pub state: TaskState,
}

impl Task {
    pub const fn new(runner: &'static dyn Run) -> Self {
        Task {
            runner,
            state: TaskState::Ready,
        }
    }

    pub fn run(&self) {
        self.runner.run();
    }
}

pub struct TaskList {
    pub tasks: &'static [Task],
}

impl TaskList {
    pub const fn new(tasks: &'static [Task]) -> Self {
        TaskList { tasks }
    }
}

#[macro_export]
macro_rules! task_list {
    ($($runner:expr),* $(,)?) => {
        {
            const COUNT: usize = <[()]>::len(&[ $( task_list!(@substitute $runner) ),* ]);
            static TASKS: [Task; COUNT] = [
                $(Task::new($runner)),*
            ];
            TaskList::new(&TASKS)
        }
    };
    (@substitute $_item:expr) => { () };
}

#[derive(Clone, Copy)]
pub enum TaskState {
    Ready,
    Running,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
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

impl TaskContext {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
