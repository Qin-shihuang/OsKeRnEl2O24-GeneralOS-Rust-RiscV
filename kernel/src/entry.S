    .section .text.entry
    .globl _start
_start:
    la sp, __stack_top
    call kernel_init

    .section .bss.stack
    .globl __stack_bottom
__stack_bottom:
    .space 4096 * 16
    .globl _stack_top
__stack_top: