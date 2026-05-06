pub fn run_installer() {
    crate::vga::set_color(0x0F, 0x05);
    crate::vga::clear_screen();
    
    crate::vga::print_str("=====================================\n");
    crate::vga::print_str("        VITEJTE V OMNIX OS           \n");
    crate::vga::print_str("=====================================\n\n");
    
    crate::vga::print_str("Detekovan novy disk.\n");
    crate::vga::print_str("Stisknete 'I' pro Instalaci systemu.\n");
    crate::vga::print_str("Stisknete 'R' pro Recovery mod.\n\n");
    crate::vga::print_str("> ");
}
