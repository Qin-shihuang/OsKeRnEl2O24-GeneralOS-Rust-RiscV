use core::arch::{asm, global_asm};

#[naked]
#[link_section = ".init.boot"]
#[export_name = "_low_entry"]
unsafe extern "C" fn _low_entry() -> ! {
    asm!(
        "   la  t1, _entry
            jr  t1
        ",
        options(noreturn)
    )
}

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_entry"]
unsafe extern "C" fn _entry(hartid: usize) -> ! {
    core::arch::asm!(
        "   mv   tp, a0",
        "   call {set_stack}",
        "   call {set_boot_page_table}",
        "   la   t0, kernel_init
            li   t1, 0xffffffff00000000
            add  t0, t0, t1
            add  sp, sp, t1
            jr   t0
        ",
        set_stack   = sym set_stack,
        set_boot_page_table = sym set_boot_page_table,
        options(noreturn),
    )
}

global_asm!(
    "   .section .data
        .align 12
    __boot_page_table_sv39:
        .quad (0x00000 << 10) | 0xcf
        .quad 0
        .quad (0x80000 << 10) | 0xcf
        .zero 8 * 507
        .quad (0x80000 << 10) | 0xcf
        .quad 0
    "
);

#[repr(C, align(4096))]
struct KernelStack([u8; 1024 * 1024]); // 1MiB stack

#[link_section = ".bss.stack"]
static mut KERNEL_STACK: core::mem::MaybeUninit<[KernelStack; 8]> =
    core::mem::MaybeUninit::uninit();


#[naked]
unsafe extern "C" fn set_stack(hartid: usize) {
    asm!(
        "   add  t0, a0, 1
            slli t0, t0, 18
            la   sp, {stack}
            add  sp, sp, t0
            ret
        ",
        stack = sym KERNEL_STACK,
        options(noreturn),
    )
}


#[naked]
unsafe extern "C" fn set_boot_page_table(hartid: usize) {
    asm!(
        "   la   t0, __boot_page_table_sv39
            srli t0, t0, 12
            li   t1, 8 << 60
            or   t0, t0, t1
            csrw satp, t0
            ret
        ",
        options(noreturn),
    )

}