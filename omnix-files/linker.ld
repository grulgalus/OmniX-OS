ENTRY(_start)

SECTIONS {
    /* Náš kernel začíná na adrese 0x8000 */
    . = 0x8000;

    .text : {
        /* Tohle zaručí, že _start bude ÚPLNĚ PRVNÍ! */
        *(.text._start)
        /* Až pak zbytek kódu */
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
