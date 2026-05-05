#[link_section = ".text._start"]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// --- 1. OVLADAČ GRAFIKY (VGA) ---
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;

fn clear_screen() {
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            unsafe {
                // Smaže obrazovku (0x0F00 = černá barva, bílý text, a mezera jako znak)
                *VGA_BUFFER.offset((y * VGA_WIDTH + x) as isize) = 0x0F00 | b' ' as u16;
            }
        }
    }
    unsafe { CURSOR_X = 0; CURSOR_Y = 0; }
}

fn print_char(c: u8) {
    unsafe {
        if c == b'\n' { // Odřádkování (Enter)
            CURSOR_Y += 1;
            CURSOR_X = 0;
        } else if c == 0x08 { // Krok zpět (Backspace)
            if CURSOR_X > 0 {
                CURSOR_X -= 1;
                *VGA_BUFFER.offset((CURSOR_Y * VGA_WIDTH + CURSOR_X) as isize) = 0x0F00 | b' ' as u16;
            }
        } else { // Obyčejný znak
            // 0x0D00 je tvá fialová barva (Light Magenta)
            *VGA_BUFFER.offset((CURSOR_Y * VGA_WIDTH + CURSOR_X) as isize) = 0x0D00 | c as u16;
            CURSOR_X += 1;
        }

        // Pokud dojdeme na konec řádku
        if CURSOR_X >= VGA_WIDTH {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        }
    }
}

fn print_str(s: &str) {
    for byte in s.bytes() {
        print_char(byte);
    }
}

// --- 2. OVLADAČ KLÁVESNICE ---

// Funkce pro čtení elektřiny z hardwarového portu procesoru
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack, preserves_flags));
    result
}

// Překlad "Scancodů" (čísel z klávesnice) na normální písmena
fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02..=0x0A => Some(b"123456789"[scancode as usize - 2]),
        0x0B => Some(b'0'),
        0x10 => Some(b'q'), 0x11 => Some(b'w'), 0x12 => Some(b'e'), 0x13 => Some(b'r'),
        0x14 => Some(b't'), 0x15 => Some(b'y'), 0x16 => Some(b'u'), 0x17 => Some(b'i'),
        0x18 => Some(b'o'), 0x19 => Some(b'p'), 0x1E => Some(b'a'), 0x1F => Some(b's'),
        0x20 => Some(b'd'), 0x21 => Some(b'f'), 0x22 => Some(b'g'), 0x23 => Some(b'h'),
        0x24 => Some(b'j'), 0x25 => Some(b'k'), 0x26 => Some(b'l'), 0x2C => Some(b'z'),
        0x2D => Some(b'x'), 0x2E => Some(b'c'), 0x2F => Some(b'v'), 0x30 => Some(b'b'),
        0x31 => Some(b'n'), 0x32 => Some(b'm'), 0x39 => Some(b' '), // Mezera
        0x1C => Some(b'\n'), // Enter
        0x0E => Some(0x08),  // Backspace
        _ => None,
    }
}

// --- 3. HLAVNÍ JÁDRO ---
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_str("FATAL ERROR: Kernel Panic!");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen(); // Připraví čistou obrazovku
    
    // UI Instalačního menu
    print_str("==================================\n");
    print_str("     OMNIX OS INSTALATOR v0.1     \n");
    print_str("==================================\n");
    print_str("Krok 1: Klávesnice inicializovana.\n\n");
    print_str("Napis neco pro otestovani PS/2 ovladace:\n> ");

    // Nekonečná smyčka - naslouchá klávesnici
    loop {
        unsafe {
            // Port 0x64 je "Status". Zeptáme se ho: Má klávesnice nová data?
            let status = inb(0x64);
            
            // Pokud je nejnižší bit 1, znamená to "Ano, někdo něco zmáčkl!"
            if (status & 1) != 0 {
                // Přečteme, JAKÉ tlačítko zmáčkl (Port 0x60)
                let scancode = inb(0x60);
                
                // Pokud je scancode pod 0x80, znamená to, že tlačítko bylo ZMÁČKNUTO (ne puštěno)
                if scancode < 0x80 {
                    if let Some(c) = scancode_to_ascii(scancode) {
                        print_char(c); // Vypíše znak na obrazovku!
                    }
                }
            }
        }
    }
}
