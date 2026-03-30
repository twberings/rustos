#![no_std]
pub mod scheduler;
pub mod task;

pub mod prelude {
    pub use crate::scheduler::*;
    pub use crate::task::*;
    pub use crate::task_list;
}
