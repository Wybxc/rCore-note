use core::panic::PanicInfo;

use crate::{debug, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        debug!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap_or(&format_args!("no message"))
        );
    } else {
        debug!(
            "Panicked: {}",
            info.message().unwrap_or(&format_args!("no message"))
        );
    }
    shutdown()
}
