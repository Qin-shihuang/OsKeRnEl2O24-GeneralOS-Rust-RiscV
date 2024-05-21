use crate::{
    mm::layout::{__text_end, __text_start},
    shutdown,
};
use core::{mem::size_of, panic::PanicInfo, ptr::addr_of};
use log::error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "\x1b[1;31mPanicked: \"{}\" at {}:{}\x1b[1;0m",
            info.message().unwrap(),
            location.file(),
            location.line(),
        );
    } else {
        error!("\x1b[1;31mPanicked: {}\x1b[1;0m", info.message().unwrap());
    }
    backtrace();
    shutdown()
}

pub fn backtrace() {
    unsafe {
        let mut current_ra = crate::arch::ra();
        let mut current_fp = crate::arch::fp();
        let mut depth = 0;
        error!("Backtrace:");
        while current_ra >= addr_of!(__text_start) as usize
            && current_ra <= addr_of!(__text_end) as usize
            && current_fp != 0
        {
            error!(
                "{:02}: RA = 0x{:016x}, FP = 0x{:016x}",
                depth,
                current_ra - size_of::<usize>(),
                current_fp
            );
            depth += 1;
            current_fp = *(current_fp as *const usize).sub(2);
            current_ra = *(current_fp as *const usize).sub(1);
        }
        error!("End of backtrace")
    }
}
