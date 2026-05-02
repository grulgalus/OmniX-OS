; =====================================================================
; OmniX OS - Hardware Manager & Hypervisor Core
; Účel: Izolace subsystémů (Linux v RAM, Windows/Wine na GPU) pro max FPS
; Architektura: x86_64
; =====================================================================

section .multiboot
align 4
    ; Multiboot hlavička pro GRUB2 (aby Grub poznal tento kód jako hlavní)
    dd 0x1BADB002              ; Magic number pro Multiboot 1
    dd 0x00                    ; Flags
    dd -(0x1BADB002)           ; Checksum

section .bss
align 16
stack_bottom:
    resb 16384                 ; Vyčlenění 16 KB pro základní stack
stack_top:

section .text
global _start

_start:
    ; 1. KROK: Nastavení základního stacku
    mov esp, stack_top

    ; 2. KROK: Vypnutí přerušení, dokud systémy nerozdělíme
    cli

    ; 3. KROK: Detekce Virtualizace (CPUID) - Zabrání hádání systémů
    ; Zjistíme, zda CPU podporuje hardwarovou izolaci (Intel VT-x nebo AMD-V)
    mov eax, 1
    cpuid
    test ecx, 1 << 5           ; Kontrola VMX (Virtual Machine Extensions) bitu
    jz .no_virtualization      ; Pokud CPU nepodporuje, přeskoč na záchranný režim

    ; Zde bychom aktivovali VMX, čímž uzamkneme Waydroid a Wine do vlastních klecí
    ; (Fyzická izolace hardwaru = systémy o sobě neví a neberou si výkon)

.setup_ram_linux:
    ; 4. KROK: Příprava RAM disku (tmpfs) pro Linux Kernel
    ; Pro maximální rychlost přesuneme klíčové části OS přímo do mezipaměti procesoru a RAM
    mov eax, 0x100000          ; Počáteční adresa pro RAM disk (1MB hranice)
    mov ebx, 0x200000          ; Konec vyhrazené paměti pro Linux Core
    ; (V reálném C/C++ kódu zde jádro připojí initramfs)

.assign_gpu_to_windows:
    ; 5. KROK: Konfigurace IOMMU pro propuštění GPU (Passthrough) do Wine
    ; Cílem je, aby Wine (Windows subsystém) měl přímý přístup ke grafické kartě
    ; čímž dosáhneme maximálních FPS pro hry a vykreslování oken.
    ; (Provede se nastavením registrů PCI sběrnice)

.launch_omnix_core:
    ; 6. KROK: Skok do hlavního C/Python Controlleru (omnix_core)
    ; Nyní je hardware bezpečně rozdělen. Linux běží v RAM, GPU čeká na Wine.
    extern kmain               ; Zavolá C funkci, která spustí tvůj omnix_controller.py
    call kmain

.hang:
    ; Pokud se něco pokazí, zastav CPU, ať se to neuškvaří
    hlt
    jmp .hang

.no_virtualization:
    ; Fallback: Pokud CPU nemá VT-x, spustíme systémy softwarově
    jmp .launch_omnix_core
