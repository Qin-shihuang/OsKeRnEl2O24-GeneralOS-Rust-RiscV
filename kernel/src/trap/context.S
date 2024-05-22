.macro SAVE_REG reg, offset
    sd \reg, \offset*8(sp)
.endm

.macro LOAD_REG reg, offset
    ld \reg, \offset*8(sp)
.endm

    .section .text
    .globl _trap
_trap:
    addi sp, sp, -8*34  # sp = sp + -8*34
    SAVE_REG x1, 1
    SAVE_REG x2, 2
    SAVE_REG x3, 3
    SAVE_REG x4, 4
    SAVE_REG x5, 5
    SAVE_REG x6, 6
    SAVE_REG x7, 7
    SAVE_REG x8, 8
    SAVE_REG x9, 9
    SAVE_REG x10, 10
    SAVE_REG x11, 11
    SAVE_REG x12, 12
    SAVE_REG x13, 13
    SAVE_REG x14, 14
    SAVE_REG x15, 15
    SAVE_REG x16, 16
    SAVE_REG x17, 17
    SAVE_REG x18, 18
    SAVE_REG x19, 19
    SAVE_REG x20, 20
    SAVE_REG x21, 21
    SAVE_REG x22, 22
    SAVE_REG x23, 23
    SAVE_REG x24, 24
    SAVE_REG x25, 25
    SAVE_REG x26, 26
    SAVE_REG x27, 27
    SAVE_REG x28, 28
    SAVE_REG x29, 29
    SAVE_REG x30, 30
    SAVE_REG x31, 31
    
    csrr t0, sstatus
    SAVE_REG t0, 32
    csrr t1, sepc
    SAVE_REG t1, 33

    mv a0, sp
    csrr a1, scause
    csrr a2, stval
    jal handle_trap
    
    .globl _restore_context
_restore_context:
    LOAD_REG t0, 32
    csrw sstatus, t0
    LOAD_REG t1, 33
    csrw sepc, t1

    LOAD_REG x1, 1
    # x2 restored later
    LOAD_REG x3, 3
    LOAD_REG x4, 4
    LOAD_REG x5, 5
    LOAD_REG x6, 6
    LOAD_REG x7, 7
    LOAD_REG x8, 8
    LOAD_REG x9, 9
    LOAD_REG x10, 10
    LOAD_REG x11, 11
    LOAD_REG x12, 12
    LOAD_REG x13, 13
    LOAD_REG x14, 14
    LOAD_REG x15, 15
    LOAD_REG x16, 16
    LOAD_REG x17, 17
    LOAD_REG x18, 18
    LOAD_REG x19, 19
    LOAD_REG x20, 20
    LOAD_REG x21, 21
    LOAD_REG x22, 22
    LOAD_REG x23, 23
    LOAD_REG x24, 24
    LOAD_REG x25, 25
    LOAD_REG x26, 26
    LOAD_REG x27, 27
    LOAD_REG x28, 28
    LOAD_REG x29, 29
    LOAD_REG x30, 30
    LOAD_REG x31, 31

    LOAD_REG x2, 2
    addi sp, sp, 8*34
    sret