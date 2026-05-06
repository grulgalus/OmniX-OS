org 0x7C00
bits 16

start:
    ; 1. Příprava paměti
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    ; 2. Povolení A20 linky (rychlý způsob)
    in al, 0x92
    or al, 2
    out 0x92, al

    ; 3. Načtení Rust Kernelu z disku (60 sektorů)
    mov ah, 0x02
    mov al, 60          
    mov ch, 0           
    mov dh, 0           
    mov cl, 2           
    mov bx, 0x8000      
    int 0x13
    jc disk_error       

    ; 4. MAGIE: PŘECHOD DO 32-BIT PROTECTED MODE
    cli                 
    lgdt [gdt_descriptor] 
    
    mov eax, cr0
    or eax, 0x1         
    mov cr0, eax
    
    jmp 0x08:start32    

disk_error:
    hlt                 

; -------------------------------------------
; GDT (Global Descriptor Table) - Zkráceno
; -------------------------------------------
align 4
gdt_start:
    dq 0x0              ; Nulový deskriptor
gdt_code:
    dw 0xFFFF, 0x0000, 0x9A00, 0x00CF ; Deskriptor kódu (0x08)
gdt_data:
    dw 0xFFFF, 0x0000, 0x9200, 0x00CF ; Deskriptor dat (0x10)
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

; -------------------------------------------
; Magický konec bootloaderu (MUSÍ BÝT ZDE!)
; -------------------------------------------
times 510-($-$$) db 0
dw 0xAA55

; -------------------------------------------
; ZDE ZAČÍNÁ 32-BITOVÝ SVĚT! (Mimo prvních 512 bytů)
; -------------------------------------------
bits 32
start32:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000    ; Nastavíme zásobník

    jmp 0x8000          ; Skok do Rustu!
