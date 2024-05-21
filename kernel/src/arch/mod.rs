#![allow(dead_code)]
use core::arch::asm;

#[inline(always)]
pub fn fp() -> usize {
    let mut value: usize;
    unsafe {
        asm!("mv {}, fp", out(reg) value);
    }
    value
}

#[inline(always)]
pub fn ra() -> usize {
    let mut value: usize;
    unsafe {
        asm!("mv {}, ra", out(reg) value);
    }
    value
}

#[inline(always)]
pub fn sp() -> usize {
    let mut value: usize;
    unsafe {
        asm!("mv {}, sp", out(reg) value);
    }
    value
}

