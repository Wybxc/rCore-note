//! 系统功能交互。

use crate::sys_call::*;

/// 将内存中缓冲区中的数据写入文件，返回成功写入的长度。
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

/// 挂起当前任务，并切换到其他任务，在切换回来后返回。
pub fn yield_() {
    sys_yield();
}

/// 退出应用程序并将返回值告知批处理系统。
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
    panic!("It should exit!");
}