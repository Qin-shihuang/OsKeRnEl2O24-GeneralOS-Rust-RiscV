// Adapted from Mankoros

#![allow(dead_code)]
use core::arch::asm;

use log::trace;
use sbi::hsm::sbi_hart_get_status;

use crate::mm::consts::PAGE_SIZE_BITS;

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

#[inline]
pub fn get_hart_id() -> usize {
    let hart_id;
    unsafe { core::arch::asm!("mv {0}, tp", out(reg) hart_id) };
    hart_id
}

#[inline]
pub fn get_hart_count() -> usize {
    let mut hart_cnt = 0;
    let mut hart_id = 0;
    loop {
        let status = sbi_hart_get_status(hart_id);
        if status.is_success() {
            hart_cnt += 1;
            hart_id += 1;
        } else {
            break;
        }
    }
    hart_cnt
}

#[inline(always)]
pub fn switch_page_table(pa: usize) -> usize {
    trace!("Switching to pagetable: 0x{:x}", pa);
    let old_page_table_ptr = riscv::register::satp::read();
    unsafe {
        riscv::register::satp::set(
            riscv::register::satp::Mode::Sv39,
            0,
            pa >> PAGE_SIZE_BITS,
        );
        riscv::asm::sfence_vma_all();
    }
    trace!("Switched to pagetable: 0x{:x}", pa);
    old_page_table_ptr.ppn() << PAGE_SIZE_BITS
}