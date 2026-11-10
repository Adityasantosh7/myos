# Entry point. QEMU "virt" loads the kernel at 0x80000000 and jumps here
# in machine mode on every hart (we only use hart 0).

.section .text.init
.global _start
_start:
    # Park every hart except hart 0
    csrr    t0, mhartid
    bnez    t0, park

    # Global pointer (needed because the compiler uses gp-relative addressing)
    .option push
    .option norelax
    la      gp, __global_pointer$
    .option pop

    # Stack
    la      sp, _stack_top

    # Zero .bss
    la      t0, _bss_start
    la      t1, _bss_end
1:
    bgeu    t0, t1, 2f
    sd      zero, 0(t0)
    addi    t0, t0, 8
    j       1b
2:
    call    kmain

park:
    wfi
    j       park
