#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

extern crate alloc;

use core::ptr::{addr_of, addr_of_mut, write_bytes};
use log::info;

pub use sbi::legacy::sbi_shutdown as shutdown;

mod arch;
#[cfg(feature = "board_qemu")]
#[path = "board/qemu.rs"]
mod board;
mod boot;
mod config;
mod console;
mod logging;
mod macros;
mod mm;
mod panic;



#[no_mangle]
extern "C" fn kernel_init(hart_id: usize, _dtb_pa: usize) -> ! {
    clear_bss();
    logging::init();
    display_banner();
    info!("GeneralOS-Rust-RiscV srarted in hart_id: {}", hart_id);
    mm::init();
    panic!()
}

fn display_banner() {
    const BANNER: &str = r#"
   ___            _  __            ___             ___      _      ___     ___     ___    _ _    
  / _ \    ___   | |/ /    ___    | _ \   _ _     | __|    | |    |_  )   / _ \   |_  )  | | |   
 | (_) |  (_-<   | ' <    / -_)   |   /  | ' \    | _|     | |     / /   | (_) |   / /   |_  _|  
  \___/   /__/_  |_|\_\   \___|   |_|_\  |_||_|   |___|   _|_|_   /___|   \___/   /___|   _|_|_  
_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""| 
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' 
           ___                                              _      ___     ___                   
   ___    / __|    ___    _ _      ___      _ _   __ _     | |    / _ \   / __|    ___           
  |___|  | (_ |   / -_)  | ' \    / -_)    | '_| / _` |    | |   | (_) |  \__ \   |___|          
  _____   \___|   \___|  |_||_|   \___|   _|_|_  \__,_|   _|_|_   \___/   |___/   _____          
_|     |_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|     |         
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'         
   ___                     _               ___      _                    __   __                 
  | _ \   _  _     ___    | |_     ___    | _ \    (_)     ___     __    \ \ / /                 
  |   /  | +| |   (_-<    |  _|   |___|   |   /    | |    (_-<    / _|    \ V /                  
  |_|_\   \_,_|   /__/_   _\__|   _____   |_|_\   _|_|_   /__/_   \__|_   _\_/_                  
_|"""""|_|"""""|_|"""""|_|"""""|_|     |_|"""""|_|"""""|_|"""""|_|"""""|_| """"|                 
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'                 
"#;
    print!("{}", BANNER);
}

fn clear_bss() {
    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;
    }

    unsafe {
        write_bytes(
            addr_of_mut!(__bss_start),
            0,
            addr_of!(__bss_end) as usize - addr_of!(__bss_start) as usize,
        );
    }
}

