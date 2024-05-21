#![allow(dead_code)]

pub const KERNEL_HEAP_SIZE : usize = 0x1_000_000; // 16 MiB

pub const MEMORY_START : usize = 0xFFFF_FFFF_8000_0000;  
pub const MEMORY_END : usize = 0xFFFF_FFFF_9000_0000;   // 256 MiB
 