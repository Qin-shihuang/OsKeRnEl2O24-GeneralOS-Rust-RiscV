// adapted from mankoros

use core::fmt;

use bitflags::bitflags;

use crate::mm::{addr::{PhysAddr, PhysPageNum}, consts::PTE_PPN_MASK, frame};

const PTEFLAGS_MASK: usize = 0x3FF;
bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct PteFlags: u16 {
        /// Valid
        const V = 1 << 0;
        /// Readable
        const R = 1 << 1;
        /// Writable
        const W = 1 << 2;
        /// Executable
        const X = 1 << 3;
        /// User mode accessible
        const U = 1 << 4;
        /// Global
        const G = 1 << 5;
        /// Accessed
        const A = 1 << 6;
        /// Dirty
        const D = 1 << 7;
        // Copy on write
        const RSW1 = 1 << 8;
        const COW = 1 << 8;
        // Reserved for software
        const RSW2 = 1 << 9;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    pub fn new(addr: PhysAddr, perm: PteFlags) -> Self {
        Self {
            bits: ((addr.floor().0 >> 2) & PTE_PPN_MASK)
                | perm.bits() as usize,
        }
    }

    pub const EMPTY: Self = Self { bits: 0 };

    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum((self.bits & PTE_PPN_MASK) >> 10)
    }

    pub fn pa(&self) -> PhysAddr {
        self.ppn().into()
    }

    pub fn flags(&self) -> PteFlags {
        PteFlags::from_bits_truncate(self.bits as u16)
    }

    pub fn is_valid(&self) -> bool {
        self.flags().contains(PteFlags::V)
    }

    pub fn is_directory(&self) -> bool {
        let mask = PteFlags::R | PteFlags::W | PteFlags::X | PteFlags::U;
        self.is_valid() && !self.flags().intersects(mask)
    }

    pub fn is_leaf(&self) -> bool {
        let mask = PteFlags::R | PteFlags::W | PteFlags::X | PteFlags::U;
        self.is_valid() && self.flags().intersects(mask)
    }

    pub fn is_readable(&self) -> bool {
        self.flags().contains(PteFlags::R)
    }

    pub fn is_writable(&self) -> bool {
        self.flags().contains(PteFlags::W)
    }

    pub fn is_executable(&self) -> bool {
        self.flags().contains(PteFlags::X)
    }

    pub fn is_user(&self) -> bool {
        self.flags().contains(PteFlags::U)
    }

    pub fn is_global(&self) -> bool {
        self.flags().contains(PteFlags::G)
    }

    pub fn is_shared(&self) -> bool {
        self.flags().contains(PteFlags::COW)
    }

    pub fn rsw_1(&self) -> bool {
        self.flags().contains(PteFlags::RSW1)
    }

    pub fn rsw_2(&self) -> bool {
        self.flags().contains(PteFlags::RSW2)
    }

    pub fn set_writable(&mut self) {
        self.bits |= PteFlags::W.bits() as usize;
    }

    pub fn clear_writable(&mut self) {
        self.bits &= !PteFlags::W.bits() as usize;
    }

    pub fn set_shared(&mut self) {
        self.bits |= PteFlags::COW.bits() as usize;
    }

    pub fn clear_shared(&mut self) {
        self.bits &= !PteFlags::COW.bits() as usize;
    }

    pub fn become_shared(&mut self, shared_writable: bool) {
        debug_assert!(!self.is_shared());
        self.set_shared();
        if !shared_writable {
            self.clear_writable();
        }
    }

    pub fn become_unique(&mut self, unique_writable: bool) {
        debug_assert!(self.is_shared());
        self.clear_shared();
        if unique_writable {
            self.set_writable();
        }
    }

    pub fn alloc_non_leaf(&mut self, perm: PteFlags) {
        debug_assert!(!self.is_valid(), "try alloc to a valid pte");
        debug_assert!(!perm.intersects(PteFlags::U | PteFlags::R | PteFlags::W));
        let frame = frame::alloc().unwrap();
        *self = Self::new(frame.into(), perm | PteFlags::V);
    }

    pub fn alloc(&mut self, perm: PteFlags) {
        debug_assert!(!self.is_valid(), "try alloc to a valid pte");
        let frame = frame::alloc().unwrap();
        *self = Self::new(frame.into(), perm | PteFlags::V | PteFlags::A | PteFlags::D);
    }

    pub fn map_frame(&mut self, perm: PteFlags, frame: PhysPageNum) {
        debug_assert!(self.is_valid(), "try map to an invalid pte");
        *self = Self::new(frame.into(), perm | PteFlags::V | PteFlags::A | PteFlags::D);
    }

    pub unsafe fn dealloc(&mut self) {
        debug_assert!(self.is_valid(), "try dealloc an invalid pte");
        frame::dealloc(self.ppn());
        self.clear();
    }

    pub unsafe fn dealloc_non_leaf(&mut self) {
        debug_assert!(self.is_directory(), "try dealloc a leaf pte");
        frame::dealloc(self.ppn());
        self.clear();
    }
}

impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "PTE @ {:p}", self)?;
        writeln!(f, "  bits: {:#018x}", self.bits)?;
        writeln!(f, "  ppn: {}", self.ppn())?;
        writeln!(f, "  flags: {:?}", self.flags())
    }
}
