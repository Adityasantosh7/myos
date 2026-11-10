OUTPUT_ARCH(riscv)
ENTRY(_start)

/* QEMU "virt" machine: RAM starts at 0x80000000 */
MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    .text : ALIGN(4K) {
        *(.text.init)
        *(.text .text.*)
    } > RAM

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    } > RAM

    .data : ALIGN(4K) {
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.*)
        *(.data .data.*)
    } > RAM

    .bss : ALIGN(8) {
        _bss_start = .;
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        _bss_end = .;
    } > RAM

    .stack (NOLOAD) : ALIGN(16) {
        _stack_bottom = .;
        . += 64K;
        _stack_top = .;
    } > RAM

    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
    }
}
