org 100h          ; Říká kompilátoru, že děláme DOS .COM soubor

start:
    ; Funkce DOSu pro vypsání textu
    mov ah, 09h
    mov dx, message   ; V NASM nepotřebujeme slovo 'offset'
    int 21h

    ; Ukončení programu a návrat do DOSu
    mov ah, 4Ch
    int 21h

message db 'Vitejte v OmniX OS (MS-DOS Edition)!$'
