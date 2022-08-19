#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;
#[no_mangle]
fn main() {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kernel should kill this application!");
    unsafe {
        asm!("sret");
    }
}