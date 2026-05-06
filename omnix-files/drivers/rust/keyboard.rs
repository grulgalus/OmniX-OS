use core::arch::asm;

unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack, preserves_flags));
    result
}

pub fn read_key() -> u8 {
    unsafe {
        if (inb(0x64) & 1) != 0 {
            let scancode = inb(0x60);
            if scancode < 0x80 {
                return scancode_to_ascii(scancode).unwrap_or(0);
            }
        }
        0
    }
}

fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02..=0x0A => Some(b"123456789"[scancode as usize - 2]),
        0x10 => Some(b'q'), 0x11 => Some(b'w'), 0x12 => Some(b'e'),
        0x1E => Some(b'a'), 0x1F => Some(b's'), 0x20 => Some(b'd'),
        0x39 => Some(b' '), 0x1C => Some(b'\n'),
        _ => None,
    }
}
