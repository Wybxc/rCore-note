use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

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

/// 输出调试信息。
#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {        
        $crate::print!(" \x1b[90m[kernel] ");
        $crate::print!($fmt $(, $($arg)+)?);
        $crate::println!("\x1b[0m");
    };
}
