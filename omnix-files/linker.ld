OUTPUT_FORMAT(binary)
ENTRY(_start)
SECTIONS {
    . = 0x7E00;
    .text : {
        *(.text._start)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
}
