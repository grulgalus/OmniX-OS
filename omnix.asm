global _start

section .data
    msg db '=== OMNIX MASTER CONTROLLER ONLINE ===', 0x0A
    msg_len equ $ - msg

section .text
_start:
    ; 1. Vytiskneme zprávu na obrazovku (pomocí systémového volání Linuxu)
    mov rax, 1          ; číslo volání pro "write" (sys_write)
    mov rdi, 1          ; kam to vypsat (1 = standardní výstup/obrazovka)
    mov rsi, msg        ; co vypsat (naše zpráva)
    mov rdx, msg_len    ; délka zprávy
    syscall             ; ŘEKNI JÁDRU, AŤ TO UDĚLÁ!

    ; 2. Spustíme linuxový terminál (Bash/Sh z Busyboxu) jako podproces
    ; Tady náš ASM kód předá dočasně velení uživateli
    
    ; (Pro zjednodušení teď uděláme jen to, že pošleme systém do pauzy, 
    ; aby se nevypnul, ale v další verzi sem přidáme spouštění GUI!)

hang:
    hlt                 ; Zastav procesor do další akce
    jmp hang            ; Smyčka donekonečna
