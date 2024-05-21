#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

extern crate alloc;

use core::arch::asm;
use log::info;

pub use sbi::legacy::sbi_shutdown as shutdown;

#[cfg(feature= "board_qemu")]
#[path ="board/qemu.rs"]
mod board;
mod config;
mod console;
mod logging;
mod macros;
mod mm;
mod panic;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _entry() -> ! {
    const KERNEL_STACK_SIZE: usize = 0x10000;

    #[link_section = ".bss.stack"]
    static mut STACK: [u8; KERNEL_STACK_SIZE] = [0; KERNEL_STACK_SIZE];

    asm!(
        "la     sp, {stack} + {stzck_size}",
        "call   kernel_init",
        stack      = sym STACK,
        stzck_size = const KERNEL_STACK_SIZE,
        options(noreturn)
    )
}

#[no_mangle]
extern "C" fn kernel_init(hart_id: usize, _dtb_pa: usize) -> ! {
    logging::init();
    display_banner();
    info!("GeneralOS-Rust-RiscV srarted in hart_id: {}", hart_id);
    mm::init();
    shutdown()
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
