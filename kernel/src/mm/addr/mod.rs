#![allow(dead_code)]

mod phys;
mod virt;

use log::warn;
pub use phys::*;
pub use virt::*;

use crate::config::{KERNEL_VIRTUAL_MEMORY_END, KERNEL_VIRTUAL_MEMORY_START, PHYSICAL_MEMORY_END, PHYSICAL_MEMORY_START};

pub fn pa2kva(pa: PhysAddr) -> VirtAddr {
    if !(PHYSICAL_MEMORY_START..PHYSICAL_MEMORY_END).contains(&pa.0) {
        warn!("Address not in physical memory range");
    }
    VirtAddr(pa.0 - PHYSICAL_MEMORY_START + KERNEL_VIRTUAL_MEMORY_START)
}

pub fn kva2pa(va: VirtAddr) -> PhysAddr {
    if !(KERNEL_VIRTUAL_MEMORY_START..KERNEL_VIRTUAL_MEMORY_END).contains(&va.0) {
        warn!("Address not in kernel virtual memory range");
    }
    PhysAddr(va.0 - KERNEL_VIRTUAL_MEMORY_START + PHYSICAL_MEMORY_START)
}

