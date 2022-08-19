//! 基于操作系统调用简单重写 `std` crate 的部分功能。
pub mod console;
pub mod sys;
pub mod process;
mod lang_items;

pub use console::*;
pub use sys::*;
pub use process::*;