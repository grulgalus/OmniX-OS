org 0x7C00
bits 16

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    in al, 0x92
    or al, 2
    out 0x92, al

    mov si, msg_prompt
    call print_str

wait_key:
    mov ah, 0x00
    int 0x16         
    
    cmp al, 0x04     ; CTRL+D
    je show_debug
    
    cmp al, 0x0D     ; ENTER
    je boot_kernel
    
    jmp wait_key

show_debug:
    mov si, msg_debug
    call print_str
    
wait_debug_key:
    mov ah, 0x00
    int 0x16
    jmp boot_kernel

boot_kernel:
    mov si, msg_booting
    call print_str

    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp 0x08:start32

print_str:
    mov ah, 0x0E
.loop:
    lodsb
    test al, al
    jz .done
    int 0x10
    jmp .loop
.done:
    ret

msg_prompt  db "OmniX OS Zavadec", 13, 10, "-> ENTER: Start OS", 13, 10, "-> CTRL+D: Debug", 13, 10, 0
msg_debug   db 13, 10, "=== DEBUG ===", 13, 10, "[OK] Bootloader na 0x7C00", 13, 10, "Stiskni libovolnou klavesu...", 13, 10, 0
msg_booting db 13, 10, "Spoustim OmniX Kernel (0x7E00)...", 13, 10, 0

align 4
gdt_start:
    dq 0x0
gdt_code:
    dw 0xFFFF, 0x0000, 0x9A00, 0x00CF
gdt_data:
    dw 0xFFFF, 0x0000, 0x9200, 0x00CF
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

bits 32
start32:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    jmp 0x7E00

times 510-($-$$) db 0
dw 0xAA55
