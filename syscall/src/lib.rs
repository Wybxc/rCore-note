//! 系统调用在内核与用户程序间的通用定义。
#![no_std]

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYS_GET_TIME: usize = 169;

pub mod fs {
    pub const FD_STDOUT: usize = 1;
}

pub mod process {
    /// 精确到微秒的时间。
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    #[repr(C)]
    pub struct TimeVal {
        pub sec: usize,
        pub usec: usize,
    }

    impl TimeVal {
        pub fn new(sec: usize, usec: usize) -> Self {
            Self { sec, usec }
        }
    }

    impl core::ops::Add<core::time::Duration> for TimeVal {
        type Output = Self;
        fn add(self, rhs: core::time::Duration) -> Self::Output {
            const MICRO_PER_SEC: usize = 1_000_000;
            let usec = self.usec + (rhs.subsec_micros() as usize);
            let sec = self.sec + (usec / MICRO_PER_SEC) + (rhs.as_secs() as usize);
            let usec = usec % MICRO_PER_SEC;
            Self { sec, usec }
        }
    }
}

pub trait SysCall {
    /// 执行系统调用。
    fn syscall(syscall_id: usize, args: [usize; 3]) -> isize;

    /// 将内存中缓冲区中的数据写入文件。（syscall ID：64）
    ///
    /// # Arguments
    /// - `fd`: 待写入文件的文件描述符；
    /// - `buf`: 内存中缓冲区的起始地址；
    /// - `len`: 内存中缓冲区的长度。
    ///
    /// # Returns
    /// 返回成功写入的长度。
    fn sys_write(fd: usize, buffer: &[u8]) -> isize;

    /// 退出应用程序并将返回值告知操作系统。（syscall ID：93）
    ///
    /// # Arguments
    /// - `xstate`: 表示应用程序的返回值。
    ///
    /// # Returns
    /// 该系统调用不应该返回。
    fn sys_exit(xstate: i32) -> isize;

    /// 应用主动交出 CPU 所有权并切换到其他应用。（syscall ID：124）
    ///
    /// # Returns
    /// 总是返回 0。
    fn sys_yield() -> isize;

    /// 获取当前的时间，保存在 TimeVal 结构体 ts 中。（syscall ID：169）
    ///
    /// # Arguments
    /// - `ts`: 返回值的存储地址。
    /// - `tz`: 时区，在目前的实现中被忽略。
    ///
    /// # Returns
    /// 返回是否执行成功，成功则返回 0    
    fn sys_get_time(ts: &mut process::TimeVal, tz: usize) -> isize;
}
