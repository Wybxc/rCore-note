use crate::{
    debug,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::{get_time_us, MICRO_PER_SEC},
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

/// get system time
pub fn sys_get_time(ts: &mut syscall::process::TimeVal, _tz: usize) -> isize {
    let time = get_time_us();
    ts.sec = time / MICRO_PER_SEC;
    ts.usec = time % MICRO_PER_SEC;
    0
}
