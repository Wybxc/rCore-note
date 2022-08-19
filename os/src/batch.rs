//! 串行批处理系统。

/// 用户栈大小。
const USER_STACK_SIZE: usize = 4096 * 2;
/// 内核栈大小。
const KERNEL_STACK_SIZE: usize = 4096 * 2;
/// 最大用户程序个数。
const MAX_APP_NUM: usize = 16;
/// 用户程序起始地址。
/// 此数值由操作系统自行决定。
const APP_BASE_ADDRESS: usize = 0x80400000;
/// 单个用户程序的大小上限。
const APP_SIZE_LIMIT: usize = 0x20000;

/// 内核栈。
#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

/// 用户栈。
#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    /// 获取栈顶（sp 指针）初始位置。由于栈是从高地址向低地址生长，所以初始栈顶是栈的地址上限。
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

/// 串行用户程序管理器。
struct AppManager {
    /// 用户程序个数。
    num_app: usize,
    /// 当前正在执行第几个用户程序。
    current_app: usize,
    /// 各用户程序的首地址。
    app_start: [usize; MAX_APP_NUM + 1],
}

use core::arch::asm;

use crate::{
    println,
    std_lite::sync::{Lazy, UPSafeCell},
    trap::TrapContext,
};

static APP_MANAGER: Lazy<UPSafeCell<AppManager>> = Lazy::new(|| unsafe {
    extern "C" {
        /// 在 `link_app.S` 中定义此地址储存着用户程序的个数。
        /// 在此地址之后，紧跟着各程序的首地址，以最后一个程序的结束地址终止。
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = num_app_ptr.read_volatile();
    let mut app_start = [0; MAX_APP_NUM + 1];
    let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
    app_start[..=num_app].copy_from_slice(app_start_raw);
    UPSafeCell::new(AppManager {
        num_app,
        current_app: 0,
        app_start,
    })
});

impl AppManager {
    /// 打印各个用户程序的信息，包括起止地址。
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    /// 加载第 `app_id` 个用户程序到用户程序基址（0x80400000）。
    /// 清空指令缓存，并将用户程序的代码拷贝到指定地址。
    ///
    /// # Safety
    /// - 用户程序的大小不能超过 `APP_SIZE_LIMIT`。
    /// - 当前不能有用户程序正在运行。
    ///
    /// # Panics
    /// 如果 `app_id` 超出范围，则会 panic。
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        // clear icache
        asm!("fence.i");
        // clear app area
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    /// 加载当前用户程序，并将内部指针指向下一个用户程序。
    pub unsafe fn load_and_move_to_next(&mut self) {
        self.load_app(self.current_app);
        self.current_app += 1;
    }
}

/// 初始化用户程序管理器，打印各个用户程序的信息。
pub fn init() {
    print_app_info();
}

/// 打印各个用户程序的信息，包括起止地址。
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

/// 运行下一个用户应用程序。
pub fn run_next_app() -> ! {
    unsafe {
        let mut app_manager = APP_MANAGER.exclusive_access();
        app_manager.load_and_move_to_next();
    };

    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        // 创建用户程序开始执行的 TrapContext，然后进入用户态。
        __restore(KERNEL_STACK.push_context(TrapContext::new_app_init(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    // 上面已经跳转入用户态，故不会执行到这里。
    panic!("Unreachable in batch::run_current_app!");
}
