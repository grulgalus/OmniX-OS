#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut CURRENT_COLOR: u8 = 0x0F;

unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("dx") port, in("al") data, options(nomem, nostack, preserves_flags));
}

unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack, preserves_flags));
    result
}

fn set_color(fg: u8, bg: u8) {
    unsafe {
        CURRENT_COLOR = (bg << 4) | fg;
    }
}

fn update_cursor() {
    unsafe {
        let pos = (CURSOR_Y * VGA_WIDTH + CURSOR_X) as u16;
        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}

fn scroll() {
    unsafe {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let from = y * VGA_WIDTH + x;
                let to = (y - 1) * VGA_WIDTH + x;
                *VGA_BUFFER.add(to) = *VGA_BUFFER.add(from);
            }
        }
        let last_line = (VGA_HEIGHT - 1) * VGA_WIDTH;
        let blank = (CURRENT_COLOR as u16) << 8 | (b' ' as u16);
        for x in 0..VGA_WIDTH {
            *VGA_BUFFER.add(last_line + x) = blank;
        }
        CURSOR_Y = VGA_HEIGHT - 1;
    }
}

fn clear_screen() {
    unsafe {
        let blank = (CURRENT_COLOR as u16) << 8 | (b' ' as u16);
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *VGA_BUFFER.add(i) = blank;
        }
        CURSOR_X = 0;
        CURSOR_Y = 0;
        update_cursor();
    }
}

fn print_char(c: u8) {
    unsafe {
        if c == b'\n' {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        } else if c == 0x08 {
            if CURSOR_X > 0 {
                CURSOR_X -= 1;
                *VGA_BUFFER.add(CURSOR_Y * VGA_WIDTH + CURSOR_X) = (CURRENT_COLOR as u16) << 8 | (b' ' as u16);
            } else if CURSOR_Y > 0 {
                CURSOR_Y -= 1;
                CURSOR_X = VGA_WIDTH - 1;
                *VGA_BUFFER.add(CURSOR_Y * VGA_WIDTH + CURSOR_X) = (CURRENT_COLOR as u16) << 8 | (b' ' as u16);
            }
        } else {
            *VGA_BUFFER.add(CURSOR_Y * VGA_WIDTH + CURSOR_X) = (CURRENT_COLOR as u16) << 8 | (c as u16);
            CURSOR_X += 1;
        }

        if CURSOR_X >= VGA_WIDTH {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        }

        if CURSOR_Y >= VGA_HEIGHT {
            scroll();
        }
        update_cursor();
    }
}

fn print_str(s: &str) {
    for byte in s.bytes() {
        print_char(byte);
    }
}

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
        0x31 => Some(b'n'), 0x32 => Some(b'm'), 0x39 => Some(b' '),
        0x1C => Some(b'\n'),
        0x0E => Some(0x08),
        _ => None,
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    set_color(0x0F, 0x04);
    clear_screen();
    print_str("FATAL KERNEL ERROR");
    loop {}
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    set_color(0x0F, 0x00);
    clear_screen();
    
    set_color(0x0B, 0x00);
    print_str("OmniX OS Core v0.3\n");
    set_color(0x0A, 0x00);
    print_str("Initializing Subsystems...\n");
    
    set_color(0x0F, 0x00);
    print_str("VGA Display: [OK]\n");
    print_str("Keyboard: [OK]\n");
    print_str("Interrupts: [PENDING]\n\n");
    
    print_str("admin@omnix:~> ");

    loop {
        unsafe {
            if (inb(0x64) & 1) != 0 {
                let scancode = inb(0x60);
                if scancode < 0x80 {
                    if let Some(c) = scancode_to_ascii(scancode) {
                        print_char(c);
                    }
                }
            }
        }
    }
}
