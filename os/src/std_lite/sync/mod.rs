//! Synchronization and interior mutability primitives

mod up;
mod lazy;

pub use up::UPSafeCell;
pub use lazy::{OnceCell, Lazy};
