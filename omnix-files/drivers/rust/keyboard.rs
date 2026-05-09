use core::arch::asm;

unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("in al, dx", out("al") data, in("dx") port);
    data
}

static mut SHIFT_PRESSED: bool = false;

// ASCII tabulky teď obsahují přímo u8 (bajty)
// V Rustu se bajt zapíše jako b'A', což je úplně to samé jako číslo 65.
const ASCII_TABLE_LOWER: [u8; 58] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8,
    9, b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 10,
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
    0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0,
    b'*', 0, b' ',
];

const ASCII_TABLE_UPPER: [u8; 58] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 8,
    9, b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', 10,
    0, b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~',
    0, b'|', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0,
    b'*', 0, b' ',
];

// Funkce teď vrací u8. Pokud není nic stisknuto, vrací 0.
pub fn read_key() -> u8 {
    unsafe {
        let status = inb(0x64);
        
        // Pokud nejsou data, vrať 0
        if (status & 0x01) == 0 {
            return 0;
        }

        // MYŠÍ FILTR (bit 5). Tohle tě zbaví těch náhodných čísel na obrazovce!
        if (status & 0x20) != 0 {
            let _trash = inb(0x60); // Přečíst a zahodit
            return 0;
        }

        let scancode = inb(0x60);

        // Detekce drženého Shiftu
        match scancode {
            0x2A | 0x36 => { SHIFT_PRESSED = true; return 0; }
            0xAA | 0xB6 => { SHIFT_PRESSED = false; return 0; }
            _ => {}
        }

        // Pokud to je puštění jiné klávesy (scancode > 0x80), ignorujeme ho
        if scancode >= 0x80 {
            return 0;
        }

        let idx = scancode as usize;
        if idx < ASCII_TABLE_LOWER.len() {
            let character = if SHIFT_PRESSED {
                ASCII_TABLE_UPPER[idx]
            } else {
                ASCII_TABLE_LOWER[idx]
            };

            return character; // Vrací přesné u8 číslo (např. 97 pro 'a')
        }
        
        0 // Pokud zmáčkneš nějakou neznámou F klávesu, pošleme 0
    }
}
