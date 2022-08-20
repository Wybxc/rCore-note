mod fs;
mod process;

pub struct SysCall;

impl syscall::SysCall for SysCall {
    #[inline]
    fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
        match syscall_id {
            syscall::SYSCALL_WRITE => {
                let buffer = unsafe { core::slice::from_raw_parts(args[1] as *const u8, args[2]) };
                Self::sys_write(args[0], buffer)
            }
            syscall::SYSCALL_EXIT => Self::sys_exit(args[0] as i32),
            syscall::SYSCALL_YIELD => Self::sys_yield(),
            syscall::SYS_GET_TIME => {
                let ts = unsafe {
                    (args[0] as *mut syscall::process::TimeVal)
                        .as_mut()
                        .unwrap()
                };
                Self::sys_get_time(ts, args[1])
            }
            _ => panic!("Unsupported syscall_id: {}", syscall_id),
        }
    }

    #[inline]
    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        fs::sys_write(fd, buffer as *const _ as *const u8, buffer.len())
    }

    #[inline]
    fn sys_exit(xstate: i32) -> isize {
        process::sys_exit(xstate)
    }

    #[inline]
    fn sys_yield() -> isize {
        process::sys_yield()
    }

    #[inline]
    fn sys_get_time(ts: &mut syscall::process::TimeVal, tz: usize) -> isize {
        process::sys_get_time(ts, tz)
    }
}
