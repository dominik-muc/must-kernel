ENTRY(_start)

SECTIONS {

    /* Tell the linker the virtual address the kernel should run at */
    . = 0xffffffff80200000;

    /* Set a symbol for later */
    __kernel_start = .;

    .text.start : AT(0x80200000) {
        *(.text.start)
        *(.text.vmem_init)
    }

    . = 0xffffffff80204000;
    .text.kernel_init : AT(0x80204000) {
        KEEP(*(.text.kernel_init))
    }

    . = 0xffffffff80204100;
    .text : AT(0x80204100) {
        *(.text .text.*)
    }

    . = 0xffffffff80210000;
    .rodata : AT(0x80210000) {
        *(.rodata .rodata.*)
    }

    . = 0xffffffff80211000;
    .data : AT(0x80211000) {
        *(.data .data.*)
    }

    . = 0xffffffff80212000;
    .eh_frame : AT(0x80212000) {
        KEEP(*(.eh_frame))
    }

    . = 0xffffffff80213000;
    .bss : AT(0x80213000) {
        __bss_start = .;
        *(.bss .bss.*)
        __bss_end = .;
    }

    . = 0xffffffff80216000;
    .stack : AT(0x80216000){

        __stack_bottom = .;

        . = . + 16M;

        __stack_top = .;
    }

    __kernel_end = .;
}
