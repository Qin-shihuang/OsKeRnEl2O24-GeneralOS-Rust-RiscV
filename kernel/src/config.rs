#![allow(dead_code)]

pub const KERNEL_HEAP_SIZE : usize = 0x1_000_000; // 16 MiB

const MEMORY_SIZE: usize = 0x1_0000_0000; // 4 GiB

pub const PHYSICAL_MEMORY_START: usize = 0x8000_0000;
pub const PHYSICAL_MEMORY_END: usize = PHYSICAL_MEMORY_START + MEMORY_SIZE;

pub const KERNEL_VIRTUAL_MEMORY_START: usize = 0xFFFF_FFFF_8000_0000;  
pub const KERNEL_VIRTUAL_MEMORY_END: usize = 0xFFFF_FFFF_FFFF_FFFF;
 