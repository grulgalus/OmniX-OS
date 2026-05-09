// Funkce, která mluví přímo s procesorem a zapisuje data ven
static inline void outb(unsigned short port, unsigned char val) {
    __asm__ volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

// Funkce, která vypíše text do sériového portu (COM1 v QEMU)
void print_serial(const char* str) {
    while (*str) {
        outb(0x3F8, *str++);
    }
}

// VSTUPNÍ BOD (tvůj Rust kernel skočí sem)
int _start() {
    print_serial("\n-----------------------------------\n");
    print_serial("[OmniX Terminal] Aplikace spustena!\n");
    print_serial("[OmniX Terminal] Cekam na syscalls...\n");
    print_serial("-----------------------------------\n");
    
    // TOTO JE KRITICKÉ: Musí to vrátit nulu, aby OS nezamrzl
    return 0; 
}
