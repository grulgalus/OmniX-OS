use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};

static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

pub fn read_key() -> u8 {
    unsafe {
        if (inb(0x64) & 1) == 0 {
            return 0;
        }

        let scancode = inb(0x60);

        match scancode {
            0x2A | 0x36 => {
                SHIFT_PRESSED.store(true, Ordering::SeqCst);
                return 0;
            }
            0xAA | 0xB6 => {
                SHIFT_PRESSED.store(false, Ordering::SeqCst);
                return 0;
            }
            _ => {}
        }

        if scancode & 0x80 != 0 {
            return 0;
        }

        let shift = SHIFT_PRESSED.load(Ordering::SeqCst);

        if shift {
            scancode_to_ascii_shift(scancode)
        } else {
            scancode_to_ascii(scancode)
        }
    }
}

fn scancode_to_ascii(scancode: u8) -> u8 {
    match scancode {
        0x01 => 27,
        0x02 => b'1',
        0x03 => b'2',
        0x04 => b'3',
        0x05 => b'4',
        0x06 => b'5',
        0x07 => b'6',
        0x08 => b'7',
        0x09 => b'8',
        0x0A => b'9',
        0x0B => b'0',
        0x0C => b'-',
        0x0D => b'=',
        0x0E => 8,
        0x0F => b'\t',
        0x10 => b'q',
        0x11 => b'w',
        0x12 => b'e',
        0x13 => b'r',
        0x14 => b't',
        0x15 => b'y',
        0x16 => b'u',
        0x17 => b'i',
        0x18 => b'o',
        0x19 => b'p',
        0x1A => b'[',
        0x1B => b']',
        0x1C => b'\n',
        0x1E => b'a',
        0x1F => b's',
        0x20 => b'd',
        0x21 => b'f',
        0x22 => b'g',
        0x23 => b'h',
        0x24 => b'j',
        0x25 => b'k',
        0x26 => b'l',
        0x27 => b';',
        0x28 => b'\'',
        0x29 => b'`',
        0x2B => b'\\',
        0x2C => b'z',
        0x2D => b'x',
        0x2E => b'c',
        0x2F => b'v',
        0x30 => b'b',
        0x31 => b'n',
        0x32 => b'm',
        0x33 => b',',
        0x34 => b'.',
        0x35 => b'/',
        0x39 => b' ',
        _ => 0,
    }
}

fn scancode_to_ascii_shift(scancode: u8) -> u8 {
    match scancode {
        0x01 => 27,
        0x02 => b'!',
        0x03 => b'@',
        0x04 => b'#',
        0x05 => b'$',
        0x06 => b'%',
        0x07 => b'^',
        0x08 => b'&',
        0x09 => b'*',
        0x0A => b'(',
        0x0B => b')',
        0x0C => b'_',
        0x0D => b'+',
        0x0E => 8,
        0x0F => b'\t',
        0x10 => b'Q',
        0x11 => b'W',
        0x12 => b'E',
        0x13 => b'R',
        0x14 => b'T',
        0x15 => b'Y',
        0x16 => b'U',
        0x17 => b'I',
        0x18 => b'O',
        0x19 => b'P',
        0x1A => b'{',
        0x1B => b'}',
        0x1C => b'\n',
        0x1E => b'A',
        0x1F => b'S',
        0x20 => b'D',
        0x21 => b'F',
        0x22 => b'G',
        0x23 => b'H',
        0x24 => b'J',
        0x25 => b'K',
        0x26 => b'L',
        0x27 => b':',
        0x28 => b'"',
        0x29 => b'~',
        0x2B => b'|',
        0x2C => b'Z',
        0x2D => b'X',
        0x2E => b'C',
        0x2F => b'V',
        0x30 => b'B',
        0x31 => b'N',
        0x32 => b'M',
        0x33 => b'<',
        0x34 => b'>',
        0x35 => b'?',
        0x39 => b' ',
        _ => 0,
    }
}
