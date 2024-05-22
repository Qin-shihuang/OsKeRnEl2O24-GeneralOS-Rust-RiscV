use core::arch::global_asm;

use log::{debug, info};
use riscv::register::{mcause::Exception, scause::{Interrupt, Scause, Trap}, sie, stvec::{self, TrapMode}};

use self::context::Context;

mod context;
mod kexception;
mod kinterrupt;

global_asm!(include_str!("ktrap.S"));

pub fn init() {
    set_kernel_trap();
    info!("Trap handler initialized.");
}

#[no_mangle]
pub extern "C" fn kernel_trap_handler(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        Trap::Interrupt(i) => kinterrupt::handle_interrupt(context, i),
        Trap::Exception(e) => kexception::handle_exception(context, e),  
    }
}

extern "C" {
    fn _kernel_trap();
}

#[inline(always)]
fn set_kernel_trap() {
    unsafe {
        stvec::write(_kernel_trap as usize, TrapMode::Direct);
        sie::set_sext();
    }
    debug!("Kernel trap vector: 0x{:x}",stvec::read().address());
}