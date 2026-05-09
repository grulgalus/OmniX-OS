static inline void outb(unsigned short port, unsigned char val) {
    __asm__ volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

void print_serial(const char* str) {
    while (*str) {
        outb(0x3F8, *str++);
    }
}

int _start() {
    print_serial("\n[OmniX Explorer] Prohledavam disk...\n");
    print_serial("[OmniX Explorer] Odesilam data zpet jadru.\n");
    
    return 0;
}
