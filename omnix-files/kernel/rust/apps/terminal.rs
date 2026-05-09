use core::arch::asm;

unsafe fn inb(port: u16) -> u8 {
    let data: u8; asm!("in al, dx", out("al") data, in("dx") port); data
}

pub fn run() {
    crate::vga::println!("\n[Terminal] HARDWARE KEY LOGGER");
    crate::vga::println!("Stiskni 3 klavesy pro navrat do OS...");

    let mut keys_pressed = 0;

    while keys_pressed < 3 {
        unsafe {
            // Kontrola, jestli klávesnice hlásí nová data na portu 0x64
            let status = inb(0x64);
            if status & 1 != 0 {
                // Přečtení samotné klávesy z portu 0x60
                let scancode = inb(0x60);
                
                // Pokud je scancode < 0x80, znamená to, že klávesa byla STISKNUTA (ne puštěna)
                if scancode < 0x80 {
                    crate::vga::println!("  > Stisknuto hw tlacitko: 0x{:02X}", scancode);
                    keys_pressed += 1;
                }
            }
        }
    }

    crate::vga::println!("[Terminal] Uvolnuji ovladani zpet jadru.");
}
