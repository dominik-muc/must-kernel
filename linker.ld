ENTRY(_start)

SECTIONS {

    /* Tell the linker the virtual address the kernel should run at */
    . = 0xffffffff80200000;

    __kernel_start = .;

    .text.start : AT(. - 0xffffffff00000000) {
        *(.text.start)
        *(.text.vmem_init)
    }

    . = 0xffffffff80204000;
    .text.kernel_init : AT(. - 0xffffffff00000000) {
        KEEP(*(.text.kernel_init))
    }

    .text : AT(. - 0xffffffff00000000) {
        *(.text .text.*)
    }

    .rodata : AT(. - 0xffffffff00000000) {
        *(.rodata .rodata.*)
    }

    .data : AT(. - 0xffffffff00000000) {
        *(.data .data.*)
    }

    .eh_frame : AT(. - 0xffffffff00000000) {
        KEEP(*(.eh_frame))
    }

    .bss : AT(. - 0xffffffff00000000) {
        __bss_start = .;
        *(.bss .bss.*)
        __bss_end = .;
    }

    . = ALIGN(16);
    .stack : AT(. - 0xffffffff00000000){

        __stack_bottom = .;

        . = . + 16M;

        __stack_top = .;
    }

    __kernel_end = .;
}
