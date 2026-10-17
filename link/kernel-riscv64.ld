OUTPUT_FORMAT(elf64-littleriscv)
OUTPUT_ARCH(riscv64)

ENTRY(start)

SECTIONS {
    /* The kernel is 100% PC-relative */

    . = 0x80200000; /* Might remove later, but this is ONLY for debugging QEMU */
    PROVIDE(__kernel_start = .);

    /*  4096-byte alignment so that sections can be managed cleanly by the MMU */

    .text : ALIGN(4096) {
        PROVIDE(__text_start = .);
        KEEP(*(.text.start));
        *(.text .text.*)
        PROVIDE(__text_end = .);
    }
    .rodata : ALIGN(4096) {
        PROVIDE(__rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(__rodata_end = .);
    }
    .data : ALIGN(4096) {
        PROVIDE(__data_start = .);
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.*)
        *(.data .data.*)
        PROVIDE(__data_end = .);
    }
    .bss : ALIGN(4096) {
        PROVIDE(__bss_start = .);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)
        PROVIDE(__bss_end = .);
    }
    .stack (NOLOAD) : ALIGN(4096) {
        PROVIDE(__boot_hart_stack_start = .);
        . += 131072;
        PROVIDE(__boot_hart_stack_end = .);
        PROVIDE(__boot_hart_trap_stack_start = .);
        . += 131072;
        PROVIDE(__boot_hart_trap_stack_end = .);
    }

    . = ALIGN(4096); /* Kernel ends on a nice page boundary */
    PROVIDE(__kernel_end = .);

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.eh_frame_hdr)
        *(.riscv.attributes)
    }
}
