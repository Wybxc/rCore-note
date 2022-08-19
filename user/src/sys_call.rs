use core::arch::asm;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

/// 进行操作系统调用。
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

/// 将内存中缓冲区中的数据写入文件。（syscall ID：64）
///
/// # Arguments
/// - `fd`: 待写入文件的文件描述符；
/// - `buf`: 内存中缓冲区的起始地址；
/// - `len`: 内存中缓冲区的长度。
///
/// # Returns
/// 返回成功写入的长度。
#[inline]
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 退出应用程序并将返回值告知批处理系统。（syscall ID：93）
///
/// # Arguments
/// - `xstate`: 表示应用程序的返回值。
///
/// # Returns
/// 该系统调用不应该返回。
#[inline]
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
