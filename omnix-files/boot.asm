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

    ; 2. Povolení A20 linky (Hardwarový trik pro zpřístupnění více než 1MB RAM)
    in al, 0x92
    or al, 2
    out 0x92, al

    ; 3. Načtení Rust Kernelu z disku
    mov ah, 0x02
    mov al, 30          ; Načteme 30 sektorů (15 KB prostoru pro náš Rust kód)
    mov ch, 0           ; Válec 0
    mov dh, 0           ; Hlava 0
    mov cl, 2           ; Začneme číst od 2. sektoru
    mov bx, 0x8000      ; Načteme to na adresu 0x8000
    int 0x13
    jc disk_error       ; Pokud se to nepovede, zastav

    ; 4. MAGIE: PŘECHOD DO 32-BIT PROTECTED MODE
    cli                 ; Vypnout BIOS přerušení (už je nikdy neuvidíme)
    lgdt [gdt_descriptor] ; Načíst tabulku GDT
    
    mov eax, cr0
    or eax, 0x1         ; Přepnout tajný bit v procesoru na 32-bit!
    mov cr0, eax
    
    ; Skok do 32-bitového prostoru (vyčistí frontu instrukcí procesoru)
    jmp 0x08:start32    

disk_error:
    hlt                 ; Zaseknutí při chybě

; -------------------------------------------
; ZDE ZAČÍNÁ 32-BITOVÝ SVĚT!
; -------------------------------------------
bits 32
start32:
    ; Musíme říct procesoru, kde teď leží data (0x10 je offset v naší GDT)
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000    ; Nastavíme bezpečný a velký zásobník

    ; Předáme velení našemu Rust Kernelu!
    jmp 0x8000          

; -------------------------------------------
; GDT (Global Descriptor Table)
; (Tohle procesor potřebuje, aby věděl, co je 32-bitový kód a co jsou data)
; -------------------------------------------
align 4
gdt_start:
    dq 0x0              ; Nulový deskriptor (povinné)
gdt_code:
    dw 0xFFFF, 0x0000, 0x9A00, 0x00CF ; Deskriptor kódu (Index 0x08)
gdt_data:
    dw 0xFFFF, 0x0000, 0x9200, 0x00CF ; Deskriptor dat (Index 0x10)
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

; Magický konec bootloaderu pro BIOS
times 510-($-$$) db 0
dw 0xAA55
