use core::panic::PanicInfo;

use crate::{exit, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap_or(&format_args!("no message"))
        );
    } else {
        println!(
            "Panicked: {}",
            info.message().unwrap_or(&format_args!("no message"))
        );
    }
    exit(-1)
}
