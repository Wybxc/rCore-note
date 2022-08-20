//! 系统功能交互。

use crate::sys_call::SysCall;
use syscall::SysCall as _;

/// 将内存中缓冲区中的数据写入文件，返回成功写入的长度。
pub fn write(fd: usize, buf: &[u8]) -> isize {
    SysCall::sys_write(fd, buf)
}

/// 挂起当前任务，并切换到其他任务，在切换回来后返回。
pub fn yield_() {
    SysCall::sys_yield();
}

/// 退出应用程序并将返回值告知批处理系统。
pub fn exit(exit_code: i32) -> ! {
    SysCall::sys_exit(exit_code);
    panic!("It should exit!");
}

/// 获取当前时间。
pub fn get_time(tz: usize) -> Result<syscall::process::TimeVal, isize> {
    let mut ts = Default::default();
    let ret = SysCall::sys_get_time(&mut ts, tz);
    if ret == 0 {
        Ok(ts)
    } else {
        Err(ret)
    }
}
