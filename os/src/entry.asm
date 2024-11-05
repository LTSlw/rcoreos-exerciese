    .section .text.entry
    .globl _start
_start:
    la sp, boot_start_top
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_start_top
boot_start_top: