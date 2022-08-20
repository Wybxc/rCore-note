#![no_std]
#![no_main]

use core::time::Duration;

use user_lib::{get_time, yield_};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() {
    let current_timer = get_time(0).unwrap();
    let wait_for = current_timer + Duration::from_secs(3);
    while get_time(0).unwrap() < wait_for {
        yield_();
    }
    println!("Test sleep OK!");
}
