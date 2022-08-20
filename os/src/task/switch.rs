use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

use super::context::TaskContext;

extern "C" {
    /// 切换任务上下文。
    /// 
    /// 此函数会挂起当前任务，直到切换回来后返回。
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
