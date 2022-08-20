//! 基于 SBI 简单重写 `std` crate 的部分功能。
pub mod console;
pub mod sync;
mod lang_items;

pub use console::*;
pub use sync::*;