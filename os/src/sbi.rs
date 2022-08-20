//! SBI 调用相关。
//! 
//! SBI 是 RISC-V Supervisor Binary Interface 规范的缩写。
//! 这个规范拥有多种实现，RustSBI是它的一种实现。
//! RISC-V架构中，存在着定义于操作系统之下的运行环境。这个运行环境不仅将引导启动RISC-V下的操作系统，
//! 还将常驻后台，为操作系统提供一系列二进制接口，以便其获取和操作硬件信息。
//! RISC-V给出了此类环境和二进制接口的规范，称为“操作系统二进制接口”，即“SBI”。
//! 
//! SBI的实现是在M模式下运行的特定于平台的固件，它将管理S、U等特权上的程序或通用的操作系统。
#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

/// 调用 SBI 函数。
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// 设置时钟中断周期。
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

/// 在控制台上打印一个字符。
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 关机。
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}