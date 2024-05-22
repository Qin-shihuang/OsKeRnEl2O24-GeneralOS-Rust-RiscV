use core::arch::global_asm;

use log::debug;
use riscv::register::{scause::{Exception, Interrupt, Scause, Trap}, sie, stvec};

use crate::timer;

use super::context::Context;

global_asm!(include_str!("context.S"));


pub fn init() {
    extern "C" {
        fn _trap();
    }
    unsafe {
        stvec::write(_trap as usize, stvec::TrapMode::Direct);
        sie::set_sext();
    }
}

#[no_mangle]
pub fn handle_trap(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(),
        _ => unimplemented_trap(scause, context, stval)
    }
}

fn breakpoint(context: &mut Context) {
    debug!("Breakpoint at {:#x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer() {
    timer::tick();
}

fn unimplemented_trap(scause: Scause, context: &mut Context, stval: usize) {
    panic!("unimplemented trap: {:?}, stval: {:#x}, sepc: {:#x}", scause.cause(), stval, context.sepc);
}