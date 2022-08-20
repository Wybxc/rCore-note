use core::arch::asm;

pub struct SysCall;

impl syscall::SysCall for SysCall {
    #[inline]
    fn syscall(id: usize, args: [usize; 3]) -> isize {
        let mut ret: isize;
        unsafe {
            asm!(
                "ecall",
                inlateout("x10") args[0] => ret,
                in("x11") args[1],
                in("x12") args[2],
                in("x17") id
            );
        }
        ret
    }

    #[inline]
    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        Self::syscall(
            syscall::SYSCALL_WRITE,
            [fd, buffer.as_ptr() as usize, buffer.len()],
        )
    }

    #[inline]
    fn sys_exit(xstate: i32) -> isize {
        Self::syscall(syscall::SYSCALL_EXIT, [xstate as usize, 0, 0])
    }

    #[inline]
    fn sys_yield() -> isize {
        Self::syscall(syscall::SYSCALL_YIELD, [0, 0, 0])
    }

    #[inline]
    fn sys_get_time(ts: &mut syscall::process::TimeVal, tz: usize) -> isize {
        Self::syscall(
            syscall::SYS_GET_TIME,
            [ts as *mut _ as usize, tz, 0],
        )
    }
}
