//! 陷入上下文相关。
use riscv::register::sstatus::{self, Sstatus, SPP};

/// 陷入上下文。
///
/// 发生陷入时，用户态的上下文保存在此结构中。
#[repr(C)]
pub struct TrapContext {
    /// 寄存器的值。
    pub x: [usize; 32],
    /// sstatus 寄存器，保存发生陷入前的特权级,以及陷入结束后返回的特权级。
    pub sstatus: Sstatus,
    /// sepc 寄存器，保存导致陷入的指令地址，以及陷入结束后继续执行的地址。
    pub sepc: usize,
}

impl TrapContext {
    /// 设置栈顶寄存器（sp，即为 x2）。
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// 创建用户程序开始执行时的 [`TrapContext`]。
    /// 
    /// # Arguments
    /// - `entry`: 用户程序的入口地址。
    /// - `sp`: 用户程序的栈顶地址。
    pub fn new_app_init(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
