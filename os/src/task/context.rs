//! 任务上下文实现。

use crate::loader::init_app_context;

/// 任务上下文，保存任务挂起前 **内核态** 寄存器的值。
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub const fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// 初始化任务上下文。
    ///
    /// 此上下文将从 `__restore` 开始继续执行，所以会立刻进入用户态启动用户程序。
    ///
    /// # Arguments
    /// - `app_id`: 用户程序 id。
    pub fn new_with_app_context(app_id: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: init_app_context(app_id),
            s: [0; 12],
        }
    }
}
