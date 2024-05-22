use crate::{mm::layout::{__text_end, __text_start}, shutdown};
use core::{mem::size_of, panic::PanicInfo, ptr::addr_of};
use alloc::{format, string::String};
use log::error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
        if let Some(location) = info.location() {
        error!(
            "\x1b[1;31mPanicked: \"{}\" at {}:{}{}\x1b[1;0m",
            info.message().unwrap(),
            location.file().split("src/").last().unwrap(),
            location.line(),
            backtrace()
        );
    } else {
        error!("\x1b[1;31mPanicked: {}{}\x1b[1;0m", info.message().unwrap(), backtrace());
    }
    shutdown()
}

fn backtrace() -> String {
    let mut result = String::new();
    unsafe {
        let mut current_ra = crate::arch::ra();
        let mut current_fp = crate::arch::fp();
        let mut depth = 0;
        result.push_str("\nBacktrace:\n");
        while current_ra >= addr_of!(__text_start) as usize
            && current_ra <= addr_of!(__text_end) as usize
            && current_fp != 0 {
            result.push_str(&format!(
                "  {:02}: RA = 0x{:016x}, FP = 0x{:016x}\n",
                depth, current_ra - size_of::<usize>(), current_fp
            ));
            current_ra = *(current_fp as *const usize).sub(1);
            current_fp = *(current_fp as *const usize).sub(2);
            depth += 1;
        }
    }
    result
}