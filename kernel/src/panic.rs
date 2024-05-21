use crate::shutdown;
use core::panic::PanicInfo;
use log::error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "\x1b[1;31mPanicked at {}:{} {}\x1b[1;0m",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("\x1b[1;31mPanicked: {}\x1b[1;0m", info.message().unwrap());
    }
    shutdown()
}