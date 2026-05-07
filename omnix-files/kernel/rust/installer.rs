use crate::vga;
use crate::keyboard;

pub fn run_installer() {
    vga::clear_screen();
    vga::print_str("======================================\n");
    vga::print_str("          VITEJTE V OMNIX OS          \n");
    vga::print_str("======================================\n\n");
    vga::print_str("Detekovan novy disk.\n");
    vga::print_str("Stisknete 'I' pro Instalaci systemu.\n");
    vga::print_str("Stisknete 'R' pro Recovery mod.\n\n> ");

    loop {
        let key = keyboard::read_key();
        if key == b'I' || key == b'i' {
            start_installation();
            break;
        } else if key == b'R' || key == b'r' {
            start_recovery();
            break;
        }
    }
}

fn start_installation() {
    vga::clear_screen();
    vga::print_str("======================================\n");
    vga::print_str("          INSTALACE OMNIX OS          \n");
    vga::print_str("======================================\n\n");
    
    vga::print_str("[1/3] Formatovani a priprava disku...\n");
    fake_delay(30000000);
    
    vga::print_str("[2/3] Kopirovani jadra systemu...\n");
    fake_delay(40000000);
    
    vga::print_str("[3/3] Nastavovani bootloaderu...\n\n");
    fake_delay(20000000);
    
    vga::print_str("Instalace dokoncena! Spoustim system...\n");
    fake_delay(40000000);
}

fn start_recovery() {
    vga::clear_screen();
    vga::print_str("=== RECOVERY MOD ===\n\n");
    vga::print_str("Zadna predchozi instalace nenalezena.\n");
    vga::print_str("System zastaven.\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

fn fake_delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}
