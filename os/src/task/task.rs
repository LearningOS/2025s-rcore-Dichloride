//! Types related to task management

use super::TaskContext;
use crate::config::MAX_APP_NUM;
use crate::syscall::SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}
/// Static array to count syscall times for each task
pub static mut SYSCALL_COUNTER: [[usize; SYSCALL_NUM]; MAX_APP_NUM] =
    [[0; SYSCALL_NUM]; MAX_APP_NUM];

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
