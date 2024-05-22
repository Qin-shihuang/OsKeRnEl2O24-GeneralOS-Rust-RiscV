use alloc::vec;
use alloc::vec::Vec;
use log::trace;

use crate::{
    arch, boot::{self, boot_page_table_pa, boot_pagetable}, config::{PHYSICAL_MEMORY_END, PHYSICAL_MEMORY_START}, mm::{addr::{kva2pa, pa2kva, PhysAddr, VirtAddr, VirtPageNum}, consts::{HUGE_PAGE_SIZE, PAGE_SIZE}, frame, layout::K_SEG_PHY_MEM_BEG}
};
use super::pte::{PageTableEntry, PteFlags};

use crate::mm::consts::PAGE_TABLE_ENTRY_COUNT as ENTRY_COUNT;

impl VirtAddr {
    fn p4_index(self) -> usize {
        (self.0 >> (12 + 27)) & (ENTRY_COUNT - 1)
    }

    fn p3_index(self) -> usize {
        (self.0 >> (12 + 18)) & (ENTRY_COUNT - 1)
    }

    fn p2_index(self) -> usize {
        (self.0 >> (12 + 9)) & (ENTRY_COUNT - 1)
    }

    fn p1_index(self) -> usize {
        (self.0 >> 12) & (ENTRY_COUNT - 1)
    }

}

pub fn enable_boot_pagetable() {
    let boot_pagetable = boot_page_table_pa().0;
    arch::switch_page_table(boot_pagetable);
}

pub fn unmap_boot_seg() {
    let boot_pagetable = boot::boot_pagetable();
    boot_pagetable[0] = 0;
    boot_pagetable[2] = 0;
}

pub fn map_kernel_phys_seg() {
    let boot_pagetable = boot::boot_pagetable();

    for i in (0..PHYSICAL_MEMORY_END).step_by(HUGE_PAGE_SIZE) {
        let pa: usize = i + PHYSICAL_MEMORY_START;
        let va = VirtAddr::from(i + K_SEG_PHY_MEM_BEG);
        boot_pagetable[va.p3_index()] = (pa >> 2) | 0xcf;
    }
}

pub struct PageTable {
    root_pa: PhysAddr,
    intrm_tables: Vec<PhysAddr>,
}

impl PageTable {
    pub fn new() -> Self {
        let root_pa = frame::alloc().unwrap().into();
        Self {
            root_pa,
            intrm_tables: vec![root_pa]
        }
    }

    pub fn new_from_boot_table() -> Self {
        let root_pa = boot_page_table_pa();
        let boot_root_pa: PhysAddr = boot::boot_page_table_pa();

        unsafe { root_pa.as_mut_page_slice().copy_from_slice(boot_root_pa.as_page_slice()) }

        Self {
            root_pa,
            intrm_tables: vec![root_pa]
        }
    }

    pub fn new_with_pa(pa: PhysAddr) -> Self {
        Self {
            root_pa: pa,
            intrm_tables: vec![pa]
        }
    }
    
    pub const fn root_pa(&self) -> PhysAddr {
        self.root_pa
    }

    pub fn map_page(&mut self, va: VirtAddr, pa: PhysAddr, perm: PteFlags) {
        let pte = PageTableEntry::new(pa, perm | PteFlags::V);
        let entry = self.get_entry_mut_or_create(va);
        *entry = pte;
    }

    pub fn unmap_page(&mut self, va: VirtAddr, dealloc: bool) -> PhysAddr {
        let entry = self.get_entry_mut(va);
        let pa = entry.pa();
        entry.clear();
        if dealloc {
            frame::dealloc(pa.into());
        }
        pa
    }


    pub fn map_region(&mut self, va: VirtAddr, pa: PhysAddr, size: usize, perm: PteFlags) {
        trace!(
            "map_region: va: {}, pa: {}, size: {:#x}, perm: {:?}",
            va,
            pa,
            size,
            perm
        );
        for offset in (0..size).step_by(PAGE_SIZE) {
            let va = va + offset;
            let pa = pa + offset;
            self.map_page(va, pa, perm);
        }
    }

    pub fn unmap_region(&mut self, va: VirtAddr, size: usize, dealloc: bool) {
        trace!("unmap_region: va: {}, size: {:#x}", va, size);
        for offset in (0..size).step_by(PAGE_SIZE) {
            let va = va + offset;
            let pa = self.unmap_page(va, dealloc);
        }
    }
    
    pub fn get_pte_copied_from_vpn(&mut self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.get_entry_mut_opt(vpn.into()).as_deref().copied()
    }

    pub fn query(&self, va: VirtAddr) -> PhysAddr {
        self.get_entry_mut(va).pa() + va.offset()
    }

        pub fn copy_table_and_mark_self_cow(&mut self, do_with_frame: impl Fn(PhysAddr)) -> Self {
        let old = self;
        let mut new = Self::new();

        let op1_iter = old.table_of_mut(old.root_pa).iter_mut();
        let np1_iter = new.table_of_mut(new.root_pa).iter_mut();

        for (op1, np1) in Iterator::zip(op1_iter, np1_iter) {
            if op1.is_leaf() {
                // Huge Page
                *np1 = *op1;
                continue;
            }
            let op2t = old.next_table_mut_opt(&op1);
            if op2t.is_none() {
                continue;
            }
            let op2_iter = op2t.unwrap().iter_mut();
            let np2_iter = new.next_table_mut_or_create(np1).iter_mut();

            for (op2, np2) in Iterator::zip(op2_iter, np2_iter) {
                let op3t = old.next_table_mut_opt(&op2);
                if op3t.is_none() {
                    continue;
                }
                let op3_iter = op3t.unwrap().iter_mut();
                let np3_iter = new.next_table_mut_or_create(np2).iter_mut();

                for (op3, np3) in Iterator::zip(op3_iter, np3_iter) {
                    if op3.is_valid() {
                        debug_assert!(op3.is_leaf());
                        if op3.is_user() {
                            do_with_frame(op3.pa());
                            op3.set_shared();
                        }
                        *np3 = *op3;
                    }
                }
            }
        }
        new
    }
}

impl PageTable {
    fn table_of<'a>(&self, pa: PhysAddr) -> &'a [PageTableEntry] {
        // use kernel_vaddr here to work after kernel remapped
        let kernel_vaddr = pa2kva(pa);
        unsafe { core::slice::from_raw_parts(kernel_vaddr.0 as _, ENTRY_COUNT) }
    }

    fn table_of_mut<'a>(&self, pa: PhysAddr) -> &'a mut [PageTableEntry] {
        // use kernel_vaddr here to work after kernel remapped
        let kernel_vaddr = pa2kva(pa);
        unsafe { core::slice::from_raw_parts_mut(kernel_vaddr.0 as _, ENTRY_COUNT) }
    }

    fn next_table_mut_opt<'a>(&self, pte: &PageTableEntry) -> Option<&'a mut [PageTableEntry]> {
        if pte.is_valid() {
            debug_assert!(pte.is_directory()); // Must be a directory
            Some(self.table_of_mut(pte.pa()))
        } else {
            None
        }
    }

    fn next_table_mut<'a>(&self, pte: &PageTableEntry) -> &'a mut [PageTableEntry] {
        debug_assert!(pte.is_valid());
        self.table_of_mut(pte.pa())
    }

    fn next_table_mut_or_create<'a>(&mut self, pte: &mut PageTableEntry) -> &'a mut [PageTableEntry] {
        if pte.is_valid() {
            self.next_table_mut(pte)
        } else {
            let frame = frame::alloc().unwrap();
            *pte = PageTableEntry::new(frame.into(), PteFlags::V);
            self.intrm_tables.push(frame.into());
            self.table_of_mut(frame.into())
        }
    }

    fn get_entry_mut_opt(&self, va: VirtAddr) -> Option<&mut PageTableEntry> {
        let p3 = self.table_of_mut(self.root_pa);
        let p3e = &mut p3[va.p3_index()];
        let p2 = self.next_table_mut_opt(p3e)?;
        let p2e = &mut p2[va.p2_index()];
        let p1 = self.next_table_mut_opt(p2e)?;
        let p1e = &mut p1[va.p1_index()];
        if p1e.is_valid() {
            Some(p1e)
        } else {
            None
        }
    }

    fn get_entry_mut(&self, va: VirtAddr) -> &mut PageTableEntry {
        let p3 = self.table_of_mut(self.root_pa);
        let p3e = &mut p3[va.p3_index()];
        let p2 = self.next_table_mut(p3e);
        let p2e = &mut p2[va.p2_index()];
        let p1 = self.next_table_mut(p2e);
        &mut p1[va.p1_index()]
    }

    fn get_entry_mut_or_create(&mut self, va: VirtAddr) -> &mut PageTableEntry {
        let p3 = self.table_of_mut(self.root_pa);
        let p3e = &mut p3[va.p3_index()];
        let p2 = self.next_table_mut_or_create(p3e);
        let p2e = &mut p2[va.p2_index()];
        let p1 = self.next_table_mut_or_create(p2e);
        &mut p1[va.p1_index()]
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        for table_pa in self.intrm_tables.iter() {
            frame::dealloc((*table_pa).into());
        }
    }
}