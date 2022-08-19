#![feature(panic_info_message)]
#![feature(const_trait_impl)]
#![feature(never_type)]
#![no_std]
#![no_main]

#[macro_use]
mod std_lite;
mod sbi;

mod batch;
mod syscall;
mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// 操作系统的入口函数。在 `entry.asm` 中调用。
#[no_mangle]
pub fn rust_main() {
    clear_bss();
    trap::init();
    batch::init();
    batch::run_next_app();
}

/// 清空 bss 段。
fn clear_bss() {
    // 在连接时找到 bss 段的起点和终点。
    extern "C" {
        fn sbss();
        fn ebss();
    }

    let bss_start = sbss as usize;
    let bss_end = ebss as usize;
    unsafe {
        core::slice::from_raw_parts_mut(bss_start as *mut u8, bss_end - bss_start).fill(0);
    }
}
