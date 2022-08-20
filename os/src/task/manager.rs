//! 任务管理器实现。

use crate::{
    debug,
    loader::{get_num_app, MAX_APP_NUM},
    std_lite::sync::{Lazy, UPSafeCell},
};

use super::{context::TaskContext, switch::__switch, TaskControlBlock, TaskStatus};

/// 任务管理器。
///
/// 其中可变部分提取到 [`UPSafeCell`] 中，目的是常量和变量分离。
pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

/// 任务管理器内部实现。
struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

static TASK_MANAGER: Lazy<TaskManager> = Lazy::new(|| {
    let num_app = get_num_app();
    let mut tasks = [TaskControlBlock {
        task_cx: TaskContext::zero_init(),
        task_status: TaskStatus::UnInit,
    }; MAX_APP_NUM];
    for (i, task) in tasks.iter_mut().enumerate().take(num_app) {
        task.task_cx = TaskContext::new_with_app_context(i);
        task.task_status = TaskStatus::Suspended;
    }
    TaskManager {
        num_app,
        inner: unsafe {
            UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })
        },
    }
});

/// 标记当前任务挂起。
pub fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// 标记当前任务结束。
pub fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// 切换到下一个任务。
pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// 运行第一个任务。
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Suspended;
        debug!("task {} suspended", current);
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
        debug!("task {} exited", current);
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let (current_task_cx_ptr, next_task_cx_ptr) = {
                let mut inner = self.inner.exclusive_access();
                let current = inner.current_task;

                debug!("switch from task {} to task {}", current, next);

                inner.tasks[next].task_status = TaskStatus::Running;
                inner.current_task = next;
                let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
                let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
                (current_task_cx_ptr, next_task_cx_ptr)
            };
            // before this, we should drop local variables that must be dropped manually
            unsafe { __switch(current_task_cx_ptr, next_task_cx_ptr) };
            // go back to user mode
        } else {
            panic!("All applications completed!");
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Suspended)
    }

    fn run_first_task(&self) -> ! {
        debug!("start first task...");
        let next_task_cx_ptr = {
            let mut inner = self.inner.exclusive_access();
            let task0 = &mut inner.tasks[0];
            task0.task_status = TaskStatus::Running;
            &task0.task_cx as *const TaskContext
        };
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
}
