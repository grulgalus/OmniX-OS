.model small
.code
org 100h

start:
    ; Funkce DOSu pro vypsani textu
    mov ah, 09h
    mov dx, offset message
    int 21h

    ; Ukonceni programu a navrat do DOSu
    mov ah, 4Ch
    int 21h

message db 'Vitejte v OmniX OS (MS-DOS Edition)!$'
end start
