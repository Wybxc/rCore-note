use core::panic::PanicInfo;

use crate::{info, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        info!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap_or(&format_args!("no message"))
        );
    } else {
        info!(
            "Panicked: {}",
            info.message().unwrap_or(&format_args!("no message"))
        );
    }
    shutdown()
}
