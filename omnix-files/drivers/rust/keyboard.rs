use core::arch::asm;

// Funkce pro čtení dat přímo z hardwarových portů procesoru
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

// Hlavní funkce, kterou voláš v main.rs
pub fn read_key() -> u8 {
    unsafe {
        // Kontrola statusu klávesnice (hardwarový port 0x64)
        // Bit 0 (hodnota 1) znamená, že je v bufferu připravená klávesa k přečtení
        if (inb(0x64) & 1) == 1 {
            
            // Přečtení samotného Scancodu z datového portu (0x60)
            let scancode = inb(0x60); 
            
            // Zajímá nás jen stisknutí (scancode < 0x80).
            // Pokud je to puštění klávesy (scancode >= 0x80), ignorujeme ho.
            if scancode < 0x80 {
                return scancode_to_ascii(scancode);
            }
        }
    }
    0 // Pokud není stisknuto nic, vrátíme 0
}

// Překladač hardwarových kódů na skutečná písmena (QWERTY rozložení)
fn scancode_to_ascii(scancode: u8) -> u8 {
    let map: [u8; 58] = [
        0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08', // 0-14
        b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',   // 15-28
        0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',             // 29-41
        0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*',          // 42-55
        0, b' '                                                                                 // 56-57 (Mezerník)
    ];

    if (scancode as usize) < map.len() {
        map[scancode as usize]
    } else {
        0
    }
}
