//! 应用进程相关。

/// 主函数返回值类型。
pub trait Termination {
    /// 获取数字形式的返回值。
    fn report(self) -> i32;
}

impl Termination for i32 {
    fn report(self) -> i32 {
        self
    }
}

impl Termination for ! {
    fn report(self) -> i32 {
        0
    }
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

impl<T: Termination> Termination for Option<T> {
    fn report(self) -> i32 {
        match self {
            Some(t) => t.report(),
            None => -1,
        }
    }
}

impl<T: Termination, E: Termination> Termination for Result<T, E> {
    fn report(self) -> i32 {
        match self {
            Ok(t) => t.report(),
            Err(e) => e.report(),
        }
    }
}
