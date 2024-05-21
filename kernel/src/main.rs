#![no_std]
#![no_main]
#![feature(panic_info_message)]


use core::arch::global_asm;


mod panic;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    panic!()
}