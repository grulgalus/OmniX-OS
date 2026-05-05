org 0x7C00
bits 16

start:
    ; Příprava paměti
    cli 
    mov ax, 0
    mov ds, ax
    mov es, ax
    sti

    ; Vypsání uvítacího textu
    mov si, message

print_char:
    lodsb             ; Načte písmeno
    cmp al, 0         ; Je to konec textu?
    je keyboard_loop  ; Pokud ano, skoč na čtení klávesnice!
    mov ah, 0x0E      ; BIOS funkce pro výpis
    int 0x10
    jmp print_char

keyboard_loop:
    ; Čekání na stisk klávesy (BIOS funkce 0x16)
    mov ah, 0x00
    int 0x16          ; Klávesa se uloží do registru AL

    ; Vypsání stisknuté klávesy na obrazovku (BIOS funkce 0x10)
    mov ah, 0x0E
    int 0x10

    ; Zpět na čekání na další klávesu
    jmp keyboard_loop

message db 'Vitejte v OmniX OS! Zkuste neco napsat: ', 0

times 510-($-$$) db 0 ; Zbytek paměti do 512 bytů vyplníme nulami
dw 0xAA55             ; Magický podpis bootovacího sektoru
