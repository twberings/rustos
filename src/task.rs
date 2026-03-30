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

#[derive(Clone, Copy)]
pub struct TaskContext {}
