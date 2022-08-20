//! 多道用户程序加载器。

use core::arch::asm;

use crate::debug;
use crate::trap::TrapContext;

/// 用户栈大小。
const USER_STACK_SIZE: usize = 4096 * 2;
/// 内核栈大小。
const KERNEL_STACK_SIZE: usize = 4096 * 2;
/// 最大用户程序个数。
pub const MAX_APP_NUM: usize = 16;
/// 用户程序起始地址。
/// 此数值由操作系统自行决定。
const APP_BASE_ADDRESS: usize = 0x80400000;
/// 单个用户程序的大小上限。
const APP_SIZE_LIMIT: usize = 0x20000;

trait Stack {
    /// 获取栈顶（sp 指针）初始位置。由于栈是从高地址向低地址生长，所以初始栈顶是栈的地址上限。
    fn get_sp(&self) -> usize;
}

/// 内核栈。
#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

/// 用户栈。
#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl Stack for KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}

impl KernelStack {
    /// 将陷入上下文压栈。
    ///
    /// # Returns
    /// 栈顶指针。
    pub fn push_context(&self, cx: TrapContext) -> usize {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe { *cx_ptr = cx };
        cx_ptr as usize
    }
}

impl Stack for UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

pub static KERNEL_STACKS: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];
pub static USER_STACKS: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

/// 获取用户程序数量。
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// 获取第 i 个用户程序的起始地址。
fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

/// 将所有 app 加载入内存空间。
///
/// 清空指令缓存，并将用户程序的代码拷贝到固定的地址，其中不同的用户程序位于不同的地址。
pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    // clear i-cache first
    unsafe { asm!("fence.i") };
    // load apps
    debug!("start loading apps");
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, APP_SIZE_LIMIT).fill(0) };
        // (base_i..base_i + APP_SIZE_LIMIT)
        //     .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
        debug!(
            "app {} loaded from [0x{:x}, 0x{:x}) to [0x{:x}, 0x{:x})",
            i,
            app_start[i],
            app_start[i + 1],
            base_i,
            base_i + src.len()
        );
    }
    debug!("finish loading apps");
}

/// get app info with entry and sp and save `TrapContext` in kernel stack
pub fn init_app_context(app_id: usize) -> usize {
    KERNEL_STACKS[app_id].push_context(TrapContext::new(
        get_base_i(app_id),
        USER_STACKS[app_id].get_sp(),
    ))
}
