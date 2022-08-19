#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(never_type)]
#![no_std]

pub mod std_lite;
mod sys_call;

/// 程序入口点。
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main().report());
}

use std_lite::*;

fn clear_bss() {
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

/// 当程序没有 main 函数时的 fallback。
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
