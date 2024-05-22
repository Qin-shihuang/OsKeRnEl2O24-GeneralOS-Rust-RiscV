use core::arch::global_asm;

use riscv::register::{scause::{Exception, Interrupt, Scause, Trap}, sie, stvec};

use crate::{println, timer};

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
    println!("Breakpoint at {:#x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer() {
    timer::tick();
}

fn unimplemented_trap(scause: Scause, context: &mut Context, stval: usize) {
    println!("unimplemented trap: {:?}, stval: {:#x}, sepc: {:#x}", scause.cause(), stval, context.sepc);
    panic!("unimplemented trap");
}