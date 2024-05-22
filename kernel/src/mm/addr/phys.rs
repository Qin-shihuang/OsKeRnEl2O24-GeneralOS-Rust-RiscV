use core::{fmt::{self, Display}, ops::{Add, AddAssign, Sub, SubAssign}};


use crate::{mask, round_up};

use super::{PAGE_SIZE, PAGE_SIZE_BITS};

const PA_WIDTH: usize = 56;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl Add<usize> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for PhysAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for PhysAddr {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Display for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PA(0x{:x})", self.0)
    }
}

impl From<usize> for PhysAddr {
    fn from(addr: usize) -> Self {
        Self(addr & mask!(PA_WIDTH))
    }
}

impl From<PhysAddr> for usize {
    fn from(addr: PhysAddr) -> Self {
        addr.0
    }
}

impl PhysAddr {
    pub fn ceil(self) -> Self {
        Self(round_up!(self.0, PAGE_SIZE))
    }

    pub fn floor(self) -> Self {
        Self(self.0 & mask!(PAGE_SIZE))
    }

    pub fn offset(self) -> usize {
        self.0 & mask!(PAGE_SIZE_BITS)
    }

    pub fn ceil_page(self) -> PhysPageNum {
        PhysPageNum(round_up!(self.0, PAGE_SIZE) >> PAGE_SIZE_BITS)
    }

    pub fn floor_page(self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SIZE_BITS)
    }
    
    pub fn as_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

impl Add<usize> for PhysPageNum {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for PhysPageNum {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for PhysPageNum {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for PhysPageNum {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Display for PhysPageNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPN(0x{:x})", self.0)
    }
}

impl From<usize> for PhysPageNum {
    fn from(addr: usize) -> Self {
        Self(addr >> 12)
    }
}

impl From<PhysPageNum> for usize {
    fn from(ppn: PhysPageNum) -> Self {
        ppn.0 << 12
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(ppn: PhysPageNum) -> Self {
        Self(ppn.0 << 12)
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(addr: PhysAddr) -> Self {
        Self(addr.0 >> 12)
    }
}