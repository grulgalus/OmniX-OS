org 0x7C00
bits 16

jmp 0x0000:start

start:
    ; Příprava paměti a zásobníku
    cli 
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    ; BIOS nám v registru DL předal číslo disku, uložíme si ho
    mov [boot_drive], dl

    ; Vypíše: "Zavadim OmniX OS..."
    mov si, msg_loading
    call print_string

    ; --- NAČTENÍ JÁDRA Z DISKU (Magie INT 13h) ---
    mov ah, 0x02       ; Funkce BIOSu: Čtení sektorů
    mov al, 4          ; Kolik sektorů načíst (4 sektory = 2 KB prostoru pro tvé jádro)
    mov ch, 0          ; Válec (Cylinder) 0
    mov dh, 0          ; Hlava (Head) 0
    mov cl, 2          ; Začni číst od 2. sektoru (v 1. sektoru je tento bootloader)
    mov dl, [boot_drive] ; Z jakého disku čteme
    mov bx, 0x8000     ; Kam do paměti to načteme (0x8000)
    int 0x13           ; Zavolej BIOS!
    jc disk_error      ; Pokud nastala chyba disku, skoč na error

    ; --- SKOK DO TVÉHO JÁDRA! ---
    jmp 0x8000         ; Freedom! Předáváme velení tvému Kernelu!

disk_error:
    mov si, msg_error
    call print_string
    hlt                ; Zaseknutí při chybě

print_string:
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp .loop
.done:
    ret

boot_drive db 0
msg_loading db 'Zavadim OmniX OS Stage 2...', 13, 10, 0
msg_error db 'Chyba cteni disku!', 13, 10, 0

; Magický podpis pro Limbo
times 510-($-$$) db 0
dw 0xAA55
