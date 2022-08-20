mod context;

use crate::syscall::SysCall;
use crate::task::suspend_current_and_run_next;
use crate::{debug, info, timer::set_next_trigger};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};
use syscall::SysCall as _;

global_asm!(include_str!("trap.S"));

/// 初始化陷入模式。
///
/// 向 `stvec` 中写入 Trap 入口地址。
pub fn init() {
    extern "C" {
        /// Trap 入口地址，见 `trap.S`。
        fn __alltraps();
    }
    debug!("init trap handler");
    unsafe {
        // 设置为 Direct 模式，即只有单一的中断入口地址。
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// 启用时钟中断。
pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
}

/// 陷入处理器，根据陷入发生的不同原因，执行不同的操作。
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = SysCall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("PageFault in application, kernel killed it.");
            panic!("Cannot continue!");
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            panic!("Cannot continue!");
            // run_next_app();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}",
            scause.cause(),
            stval
        ),
    }
    cx
}

pub use context::TrapContext;
