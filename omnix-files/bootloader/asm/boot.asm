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

    mov [BOOT_DRIVE], dl

    mov si, msg_boot
    call print_string

    in al, 0x92
    or al, 2
    out 0x92, al

load_kernel:
    mov ah, 0x02
    mov al, 60
    mov ch, 0
    mov dh, 0
    mov cl, 2
    mov bx, 0x8000
    int 0x13
    jc disk_retry
    jmp protected_mode

disk_retry:
    dec byte [RETRIES]
    jz disk_error
    xor ax, ax
    int 0x13
    jmp load_kernel

print_string:
    mov ah, 0x0E
.loop:
    lodsb
    test al, al
    jz .done
    int 0x10
    jmp .loop
.done:
    ret

protected_mode:
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp 0x08:start32

disk_error:
    mov si, msg_err
    call print_string
    cli
    hlt

BOOT_DRIVE db 0
RETRIES db 5
msg_boot db "Loading OmniX OS...", 13, 10, 0
msg_err db "Disk Error!", 0

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

times 510-($-$$) db 0
dw 0xAA55

bits 32
start32:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    jmp 0x8000
