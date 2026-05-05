org 0x8000   ; TADY JE TA ZMĚNA! Běžíme na nové adrese v paměti!
bits 16

start_kernel:
    mov si, msg_welcome
    call print_string

prompt_loop:
    mov si, msg_prompt
    call print_string
    mov di, input_buffer
    mov cx, 0

input_loop:
    mov ah, 0x00
    int 0x16

    cmp al, 0x0D        ; ENTER
    je enter_pressed
    cmp al, 0x08        ; BACKSPACE
    je backspace_pressed
    cmp cx, 50          ; Zvýšili jsme limit znaků (MÁME MÍSTO!)
    jge input_loop

    stosb
    inc cx
    mov ah, 0x0E
    int 0x10
    jmp input_loop

backspace_pressed:
    cmp cx, 0
    je input_loop
    dec cx
    dec di
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
    stosb
    mov si, msg_newline
    call print_string

    mov si, input_buffer
    cmp byte [si], 0
    je prompt_loop

    ; --- PŘÍKAZY ---
    mov cx, 5
    mov si, input_buffer
    mov di, cmd_help
    repe cmpsb
    je do_help

    mov si, msg_unknown
    call print_string
    jmp prompt_loop

do_help:
    mov si, msg_help
    call print_string
    jmp prompt_loop

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

msg_welcome db 'Vitej v JADRE OmniX OS s verzi 0.3!', 13, 10, 'Konecne neomezeny prostor pro kod!', 13, 10, 0
msg_prompt  db 'OmniX-OS/root$ ', 0
msg_newline db 13, 10, 0
msg_unknown db 'Prikaz neexistuje. (Zkus help)', 13, 10, 0
msg_help    db 'Dostupne prikazy: help', 13, 10, 0
cmd_help    db 'help', 0

input_buffer times 64 db 0
; !!! Všimni si, že na konci KERNELU už NENÍ to "times 510-..." a 0xAA55 !!!
