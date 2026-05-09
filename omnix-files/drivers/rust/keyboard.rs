use core::arch::asm;

// Funkce pro čtení z portu
unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("in al, dx", out("al") data, in("dx") port);
    data
}

// PAMĚŤ PRO SHIFT (Uloží si, jestli držíš klávesu)
static mut SHIFT_PRESSED: bool = false;

// 1. ZÁKLADNÍ ASCII TABULKA (Bez shiftu)
const ASCII_TABLE_LOWER: [char; 58] = [
    '\0', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\x08',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    '\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\0',
    '*', '\0', ' ',
];

// 2. SHIFT ASCII TABULKA (S drženým shiftem)
const ASCII_TABLE_UPPER: [char; 58] = [
    '\0', '\x1B', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\x08',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    '\0', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    '\0', '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', '\0',
    '*', '\0', ' ',
];

// 3. FUNKCE, KTERÁ ČTE HARDWARE A VRACÍ ROVNOU PÍSMENKO
pub fn read_key() -> Option<char> {
    unsafe {
        let status = inb(0x64);
        
        // Nejsou žádná data
        if (status & 0x01) == 0 {
            return None;
        }

        // FIX NA MYŠ! (Pokud je bit 5 roven 1, jsou to data myši -> ZAHODIT)
        if (status & 0x20) != 0 {
            let _trash = inb(0x60); 
            return None;
        }

        // Přečteme scancode z klávesnice
        let scancode = inb(0x60);

        // DETEKCE SHIFTU (Scancody: 0x2A = Levý Shift, 0x36 = Pravý Shift)
        // Když klávesu pustíš, přičte se k jejímu kódu 0x80 (takže 0xAA a 0xB6)
        match scancode {
            0x2A | 0x36 => { SHIFT_PRESSED = true; return None; }
            0xAA | 0xB6 => { SHIFT_PRESSED = false; return None; }
            _ => {}
        }

        // Zajímají nás jen "stisky" (hodnoty pod 0x80). 
        // Všechna ostatní uvolnění kláves ignorujeme.
        if scancode < 0x80 {
            let idx = scancode as usize;
            
            if idx < ASCII_TABLE_LOWER.len() {
                // Vybereme tabulku podle toho, jestli se drží Shift
                let character = if SHIFT_PRESSED {
                    ASCII_TABLE_UPPER[idx]
                } else {
                    ASCII_TABLE_LOWER[idx]
                };

                if character != '\0' {
                    return Some(character);
                }
            }
        }
        
        None
    }
}
