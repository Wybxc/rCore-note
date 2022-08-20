//! 多任务上下文和任务切换。

use self::{
    context::TaskContext,
    manager::{mark_current_exited, mark_current_suspended, run_next_task},
};

mod context;
mod manager;
mod switch;

/// 任务生命周期状态。
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    /// 未初始化。
    UnInit,
    /// 挂起中，准备运行。
    Suspended,
    /// 运行中。
    Running,
    /// 已结束。
    Exited,
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}

/// 结束当前任务，并切换到下一个任务。
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

/// 挂起当前任务，并切换到下一个任务。
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub use self::manager::run_first_task;