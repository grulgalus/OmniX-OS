use core::arch::asm;

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut CURRENT_COLOR: u8 = 0x0F;

pub unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("dx") port, in("al") data, options(nomem, nostack, preserves_flags));
}

pub fn set_color(fg: u8, bg: u8) {
    unsafe { CURRENT_COLOR = (bg << 4) | fg; }
}

pub fn clear_screen() {
    unsafe {
        let blank = (CURRENT_COLOR as u16) << 8 | (b' ' as u16);
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *VGA_BUFFER.add(i) = blank;
        }
        CURSOR_X = 0;
        CURSOR_Y = 0;
    }
}

pub fn print_char(c: u8) {
    unsafe {
        if c == b'\n' {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        } else {
            *VGA_BUFFER.add(CURSOR_Y * VGA_WIDTH + CURSOR_X) = (CURRENT_COLOR as u16) << 8 | (c as u16);
            CURSOR_X += 1;
        }
        if CURSOR_X >= VGA_WIDTH {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        }
    }
}

pub fn print_str(s: &str) {
    for byte in s.bytes() {
        print_char(byte);
    }
}
