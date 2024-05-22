#![allow(dead_code)] // TODO

use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::info;
use spin::Mutex;

use crate::{prev_pow_of_2, print, println};

use super::addr::{PhysAddr, PhysPageNum, PAGE_SIZE};

const ORDER: usize = 32;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<ORDER>> =
        Mutex::new(FrameAllocator::new());
}

pub struct FrameAllocator<const ORDER: usize> {
    free_list: [Vec<PhysPageNum>; ORDER],
    total: usize,
    allocated: usize,
}

impl<const ORDER: usize> FrameAllocator<ORDER> {
    const fn new() -> Self {
        const NEW_VEC: Vec<PhysPageNum> = Vec::new();
        Self {
            free_list: [NEW_VEC; ORDER],
            total: 0,
            allocated: 0,
        }
    }

    pub fn init(&mut self, start: PhysAddr, end: PhysAddr) {
        assert!(start.0 < end.0);
        let start = start.ceil_page();
        let end = end.floor_page();
        let mut current = start;
        while current < end {
            let lowbit = 1 << current.0.trailing_zeros();
            let size = usize::min(lowbit, prev_pow_of_2!(end.0 - current.0));
            let order = size.trailing_zeros() as usize;
            self.free_list[order].push(current);
            current += size;
        }
        self.total = end.0 - start.0;
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<PhysPageNum> {
        if size == 0 || align == 0 || size > self.total || align > self.total {
            return None;
        }
        assert!(size.is_power_of_two());
        assert!(align.is_power_of_two());
        let order = size.trailing_zeros() as usize;
        let align_order = align.trailing_zeros() as usize;
        let start_order = usize::max(order, align_order);
        for i in start_order..ORDER {
            if !self.free_list[i].is_empty() {
                for j in ((order + 1)..=i).rev() {
                    let ppn = self.free_list[j]
                        .pop()
                        .expect("There should be some free frames");
                    self.free_list[j - 1].push(PhysPageNum(ppn.0 + (1 << (j - 1)))); // This is the buddy frame
                    self.free_list[j - 1].push(ppn); // This is the allocated frame, which matches the align
                }
                let ppn = self.free_list[order]
                    .pop()
                    .expect("There should be some free frames");
                self.allocated += 1 << order;
                return Some(ppn);
            }
        }
        None
    }

    pub fn dealloc(&mut self, start: PhysPageNum, size: usize) {
        assert!(size.is_power_of_two());
        assert!(start.0 & (size - 1) == 0);
        let order = size.trailing_zeros() as usize;
        let mut ppn = start;
        let mut order = order;
        while order < ORDER - 1 {
            let buddy = PhysPageNum(ppn.0 ^ (1 << order));
            let mut found = false;
            for block in &self.free_list[order] {
                if *block == buddy {
                    found = true;
                    break;
                }
            }
            if found {
                self.free_list[order].retain(|x| *x != buddy);
                ppn = PhysPageNum(ppn.0 & buddy.0);
                order += 1;
            } else {
                break;
            }
        }
        self.free_list[order].push(ppn);
        self.allocated -= size;
    }

    fn debug_print(&self) {
        println!("FrameAllocator {{");
        println!("  total: {}, allocated: {}", self.total, self.allocated);
        println!("  free_list: [");
        for i in 0..ORDER {
            if !self.free_list[i].is_empty() {
                print!("    order {}: ", i);
                for ppn in &self.free_list[i] {
                    print!("{} ", ppn);
                }
                println!();
            } else {
                println!("    order {}: empty", i);
            }
        }
        println!("  ]");
        println!("}}");
    }
}

pub struct Frame {
    pub ppn: PhysPageNum,
}

impl Frame {
    fn start_address(&self) -> PhysAddr {
        PhysAddr::from(self.ppn)
    }
}

unsafe fn clear_frame(frame: &Frame, size: usize) {
    let ptr = frame.start_address().0 as *mut u8;
    ptr.write_bytes(0, PAGE_SIZE * size);
}

pub fn debug_print() {
    FRAME_ALLOCATOR.lock().debug_print();
}

pub fn init(start: PhysAddr, end: PhysAddr) {
    FRAME_ALLOCATOR.lock().init(start, end);
    info!(
        "Initialized frame allocator with {} frames in total.",
        FRAME_ALLOCATOR.lock().total
    );
}

pub fn alloc_frames(size: usize, align: usize) -> Option<Frame> {
    let frame = FRAME_ALLOCATOR
        .lock()
        .alloc(size, align)
        .map(|ppn| Frame { ppn });
    if let Some(ref frame) = frame {
        unsafe {
            clear_frame(frame, size);
        }
    }
    frame
}

pub fn dealloc_frames(start: PhysPageNum, size: usize) {
    FRAME_ALLOCATOR.lock().dealloc(start, size);
}
