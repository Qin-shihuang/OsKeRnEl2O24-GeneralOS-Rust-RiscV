use core::fmt::{self, Debug};

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Context {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

impl Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context {{ regs: [")?;
        for i in 0..32 {
            write!(f, "x{}: {:#x}, ", i, self.regs[i])?;
        }
        write!(f, "], sstatus: {:#x}, sepc: {:#x} }}", self.sstatus, self.sepc)
    }
}