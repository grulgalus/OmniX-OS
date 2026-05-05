org 0x7C00
bits 16

start:
    ; Příprava paměti
    cli 
    xor ax, ax
    mov ds, ax
    mov es, ax
    sti

    ; Vypsání uvítací zprávy při startu
    mov si, msg_welcome
    call print_string

prompt_loop:
    ; Vypsání příkazového řádku (prompt)
    mov si, msg_prompt
    call print_string

    ; Příprava bufferu (paměti) pro tvůj příkaz
    mov di, input_buffer
    mov cx, 0           ; Počítadlo znaků

input_loop:
    mov ah, 0x00
    int 0x16            ; Čekání na klávesu -> uloží se do AL

    cmp al, 0x0D        ; Zmáčkl uživatel ENTER?
    je enter_pressed

    cmp al, 0x08        ; Zmáčkl uživatel BACKSPACE?
    je backspace_pressed

    ; Ochrana proti přetečení (max 20 znaků)
    cmp cx, 20
    jge input_loop

    ; Uložení znaku do bufferu a vypsání na obrazovku
    stosb               ; Uloží znak do paměti
    inc cx              ; Zvýší počítadlo
    mov ah, 0x0E
    int 0x10            ; Vypíše znak
    jmp input_loop

backspace_pressed:
    cmp cx, 0           ; Jsme na začátku? (Není co mazat)
    je input_loop
    dec cx              ; Snížíme počítadlo
    dec di              ; Vrátíme se v paměti o krok zpět
    ; Vizuální smazání z obrazovky (krok zpět, mezera, krok zpět)
    mov ah, 0x0E
    mov al, 0x08
    int 0x10
    mov al, ' '
    int 0x10
    mov al, 0x08
    int 0x10
    jmp input_loop

enter_pressed:
    mov al, 0
    stosb               ; Ukončení textu nulou

    ; Odřádkování
    mov si, msg_newline
    call print_string

    ; Zkontrolování, jestli uživatel nezadal prázdný řádek
    mov si, input_buffer
    cmp byte [si], 0
    je prompt_loop

    ; --- TADY BUDOU TVOJE PŘÍKAZY ---
    
    ; Zkusíme, jestli uživatel nenapsal "help"
    mov cx, 5
    mov si, input_buffer
    mov di, cmd_help
    repe cmpsb
    je do_help

    ; Pokud příkaz neznáme, vypíšeme chybu:
    mov si, msg_unknown
    call print_string
    jmp prompt_loop

do_help:
    mov si, msg_help
    call print_string
    jmp prompt_loop

; --- POMOCNÉ FUNKCE ---

print_string:
.loop:
    lodsb               ; Načte znak do AL
    or al, al           ; Je to nula (konec textu)?
    jz .done
    mov ah, 0x0E
    int 0x10            ; Vypíše znak
    jmp .loop
.done:
    ret

; --- DATA A TEXTY ---

msg_welcome db 'Vitej v Omnix OS s verzi 0.2!', 13, 10, 0
msg_prompt  db 'OmniX-OS/root$ ', 0
msg_newline db 13, 10, 0
msg_unknown db 'Chyba: Souborovy system (FAT) neni nacten. Nelze pouzit mv, cp ani rm!', 13, 10, 0
msg_help    db 'Dostupne prikazy: help', 13, 10, 'Pro prikazy nano, mv a apt je nutne jadro (Kernel).', 13, 10, 0

cmd_help    db 'help', 0

; Zde se ukládá to, co píšeš:
input_buffer times 24 db 0

; Magické ukončení boot sektoru
times 510-($-$$) db 0
dw 0xAA55
