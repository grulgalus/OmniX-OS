org 0x7C00        ; Přesně sem BIOS načítá operační systém
bits 16           ; Jsme ve starém dobrém 16-bitovém režimu

start:
    ; Příprava paměti
    cli 
    mov ax, 0
    mov ds, ax
    mov es, ax
    sti

    ; Načtení textu
    mov si, message

print_char:
    lodsb             ; Načte jedno písmeno do AL
    cmp al, 0         ; Je to konec textu (nula)?
    je done           ; Pokud ano, přeskoč na konec
    mov ah, 0x0E      ; BIOS funkce: Vypiš znak na obrazovku
    int 0x10          ; Zavolej BIOS
    jmp print_char    ; Opakuj pro další písmeno

done:
    hlt               ; Uspi procesor
    jmp done          ; Zaseknutí v nekonečné smyčce (aby se to nevyplo)

message db 'Vitejte v OmniX OS! (Verze 0.1, Vlastni Bootloader)', 0

times 510-($-$$) db 0 ; Zbytek paměti do 512 bytů vyplníme nulami
dw 0xAA55             ; MAGICKÝ PODPIS! Tohle řekne Limbu: "Jsem bootovací disk!"
