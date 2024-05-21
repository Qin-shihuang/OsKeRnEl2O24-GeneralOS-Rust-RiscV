use core::{fmt::{self, Display}, ops::Add};

use crate::mask;

pub const PAGE_SIZE: usize = 4096;
const PAGE_SIZE_BITS: usize = 12;
// We follow SV39 paging scheme
const PA_WIDTH: usize = 56;
const VA_WIDTH: usize = 64;
const PPN_WIDTH: usize = 56;
const VPN_WIDTH: usize = 39;

#[derive(Clone, Copy)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy)]
pub struct VirtPageNum(pub usize);

impl Add<usize> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Display for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PA(0x{:x})", self.0)
    }
}

impl Display for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VA(0x{:x})", self.0)
    }
}

impl Display for PhysPageNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPN(0x{:x})", self.0)
    }
}

impl Display for VirtPageNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VPN(0x{:x})", self.0)
    }
}

impl From<usize> for PhysAddr {
    fn from(addr: usize) -> Self {
        Self(addr & mask!(PA_WIDTH))
    }
}

impl From<usize> for VirtAddr {
    fn from(addr: usize) -> Self {
        Self(addr & mask!(VA_WIDTH))
    }
}

impl From<usize> for PhysPageNum {
    fn from(addr: usize) -> Self {
        Self(addr & mask!(PPN_WIDTH))
    }
}

impl From<usize> for VirtPageNum {
    fn from(addr: usize) -> Self {
        Self(addr & mask!(VPN_WIDTH))
    }
}

impl From<PhysAddr> for usize {
    fn from(addr: PhysAddr) -> Self {
        addr.0
    }
}

impl From<VirtAddr> for usize {
    fn from(addr: VirtAddr) -> Self {
        addr.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(addr: PhysPageNum) -> Self {
        addr.0
    }
}

impl From<VirtPageNum> for usize {
    fn from(addr: VirtPageNum) -> Self {
        addr.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(addr: PhysAddr) -> Self {
        Self(addr.0 >> PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(addr: VirtAddr) -> Self {
        Self(addr.0 >> PAGE_SIZE_BITS)
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(addr: PhysPageNum) -> Self {
        Self(addr.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(addr: VirtPageNum) -> Self {
        Self(addr.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddr {
    pub fn ceil(self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) >> PAGE_SIZE_BITS)
    }

    pub fn floor(self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SIZE_BITS)
    }
}