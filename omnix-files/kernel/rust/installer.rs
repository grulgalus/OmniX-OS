pub fn run_installer() {
    crate::drivers::vga::set_color(0x0F, 0x05);
    crate::drivers::vga::clear_screen();
    
    crate::drivers::vga::print_str("=====================================\n");
    crate::drivers::vga::print_str("        VITEJTE V OMNIX OS           \n");
    crate::drivers::vga::print_str("=====================================\n\n");
    
    crate::drivers::vga::print_str("Detekovan novy disk.\n");
    crate::drivers::vga::print_str("Stisknete 'I' pro Instalaci systemu.\n");
    crate::drivers::vga::print_str("Stisknete 'R' pro Recovery mod.\n\n");
    crate::drivers::vga::print_str("> ");
}
