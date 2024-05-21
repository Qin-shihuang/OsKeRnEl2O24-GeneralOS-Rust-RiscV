#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

use core::arch::asm;

use log::info;
pub use sbi::legacy::sbi_shutdown as shutdown;

mod console;
mod logging;
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
    info!("MOS-Rust-RiscV srarted in hart_id: {}", hart_id);
    shutdown()
}

fn display_banner() {
    const BANNER: &str = r#"
            ___            _  __            ___             ___      _      ___     ___     ___    _ _            
    o O O  / _ \    ___   | |/ /    ___    | _ \   _ _     | __|    | |    |_  )   / _ \   |_  )  | | |     ___   
   o      | (_) |  (_-<   | ' <    / -_)   |   /  | ' \    | _|     | |     / /   | (_) |   / /   |_  _|   |___|  
  TS__[O]  \___/   /__/_  |_|\_\   \___|   |_|_\  |_||_|   |___|   _|_|_   /___|   \___/   /___|   _|_|_   _____  
 {======|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|     | 
./o--000'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' 
 __  __    ___     ___             ___                     _               ___      _                    __   __  
|  \/  |  / _ \   / __|    ___    | _ \   _  _     ___    | |_     ___    | _ \    (_)     ___     __    \ \ / /  
| |\/| | | (_) |  \__ \   |___|   |   /  | +| |   (_-<    |  _|   |___|   |   /    | |    (_-<    / _|    \ V /   
|_|__|_|  \___/   |___/   _____   |_|_\   \_,_|   /__/_   _\__|   _____   |_|_\   _|_|_   /__/_   \__|_   _\_/_   
_|"""""|_|"""""|_|"""""|_|     |_|"""""|_|"""""|_|"""""|_|"""""|_|     |_|"""""|_|"""""|_|"""""|_|"""""|_| """"|  
"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'  
"#;
    print!("{}", BANNER);
}
