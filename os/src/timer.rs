//! 硬件时钟相关。

use riscv::register::time;

use crate::sbi::set_timer;

#[cfg(feature = "k210")]
pub const CLOCK_FREQ: usize = 403000000 / 62;
#[cfg(feature = "qemu")]
pub const CLOCK_FREQ: usize = 12500000;

const TICKS_PER_SEC: usize = 100;

pub const MICRO_PER_SEC: usize = 1_000_000;

/// 获取 `mtime` 时钟寄存器的值。
pub fn get_time() -> usize {
    time::read()
}

/// 设置下一次时钟中断的触发。
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/// 获取以微秒为单位的时间。
pub fn get_time_us() -> usize {
    get_time() / (CLOCK_FREQ / MICRO_PER_SEC)
}
