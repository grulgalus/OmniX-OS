use crate::vga;
use crate::keyboard;
use crate::omxapk;

pub fn start() {
    vga::set_color(0x0F, 0x01); 
    vga::clear_screen();
    
    vga::print_str("======================================\n");
    vga::print_str("          OMNIX OS - PLOCHA           \n");
    vga::print_str("======================================\n\n");
    vga::print_str("1. Nastaveni systemu\n");
    vga::print_str("2. Prikazovy radek\n");
    vga::print_str("3. Vypnout PC\n\n");
    vga::print_str("Vyberte aplikaci (1-3): ");

    loop {
        let key = keyboard::read_key();
        
        if key == b'1' {
            omxapk::run_application("system_ui.omxapk");
            fake_delay(50000000);
            start();
            break;
        } else if key == b'2' {
            omxapk::run_application("terminal.omxapk");
            fake_delay(50000000);
            start();
            break;
        } else if key == b'3' {
            vga::clear_screen();
            vga::print_str("Vypinani OmniX OS...\n");
            loop { unsafe { core::arch::asm!("hlt"); } }
        }
    }
}

fn fake_delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}
