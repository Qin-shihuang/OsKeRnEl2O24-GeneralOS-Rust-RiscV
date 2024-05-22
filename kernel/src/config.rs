#![allow(dead_code)]

pub const KERNEL_HEAP_SIZE : usize = 0x1_000_000; // 16 MiB

pub const PHYSICAL_MEMORY_START: usize = 0x8000_0000;
pub const PHYSICAL_MEMORY_END: usize = 0x9000_0000;
pub const KERNEL_VIRTUAL_MEMORY_START : usize = 0xFFFF_FFFF_8000_0000;  
pub const KERNEL_VIRTUAL_MEMORY_END : usize = 0xFFFF_FFFF_9000_0000;   // 256 MiB
 