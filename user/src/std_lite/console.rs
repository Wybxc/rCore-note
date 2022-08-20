//! 控制台输出。

use super::sys::*;
use core::fmt::{self, Write};

const STDOUT: usize = 1;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

/// 打印格式化字符串。
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 打印一个字符串。
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::std_lite::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 打印一个字符串并换行。
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::std_lite::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
