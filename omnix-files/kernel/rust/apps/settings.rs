use core::arch::asm;

// Pomocné funkce pro komunikaci s hardwarem
unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("al") data, in("dx") port);
}
unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("in al, dx", out("al") data, in("dx") port);
    data
}

pub fn run() {
    crate::vga::println!("\n[Nastaveni] Testuji hardware PC Speaker...");

    let freq = 440; // Frekvence 440 Hz
    let div = 1193180 / freq;

    unsafe {
        // Nastavení zvukového čipu
        outb(0x43, 0xb6);
        outb(0x42, (div & 0xFF) as u8);
        outb(0x42, (div >> 8) as u8);

        // Zapnutí reproduktoru
        let tmp = inb(0x61);
        if tmp != (tmp | 3) {
            outb(0x61, tmp | 3);
        }

        // Chvíli počkáme (aby tón hrál - primitivní delay)
        for _ in 0..5_000_000 {
            asm!("nop");
        }

        // Vypnutí reproduktoru
        let tmp = inb(0x61) & 0xFC;
        outb(0x61, tmp);
    }

    // crate::vga::println!("[Nastaveni] Audio test dokocen!");
}
