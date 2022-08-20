use crate::{
    debug,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> isize {
    debug!("Application exited with code {}", exit_code);
    exit_current_and_run_next();
    0
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}
