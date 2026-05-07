use core::arch::asm;

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut COLOR: u8 = 0x5F; // Výchozí: fialové pozadí, bílý text

pub fn set_color(fg: u8, bg: u8) {
    unsafe { COLOR = (bg << 4) | fg; }
}

pub fn clear_screen() {
    unsafe {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let offset = (y * VGA_WIDTH + x) * 2;
                *VGA_BUFFER.add(offset) = b' ';
                *VGA_BUFFER.add(offset + 1) = COLOR;
            }
        }
        CURSOR_X = 0;
        CURSOR_Y = 0;
        update_cursor();
    }
}

pub fn print_char(c: u8) {
    unsafe {
        if c == b'\n' {
            CURSOR_X = 0;
            CURSOR_Y += 1;
        } else if c == 8 { // Backspace
            if CURSOR_X > 0 {
                CURSOR_X -= 1;
            }
            let offset = (CURSOR_Y * VGA_WIDTH + CURSOR_X) * 2;
            *VGA_BUFFER.add(offset) = b' ';
            *VGA_BUFFER.add(offset + 1) = COLOR;
        } else {
            let offset = (CURSOR_Y * VGA_WIDTH + CURSOR_X) * 2;
            *VGA_BUFFER.add(offset) = c;
            *VGA_BUFFER.add(offset + 1) = COLOR;
            CURSOR_X += 1;
            if CURSOR_X >= VGA_WIDTH {
                CURSOR_X = 0;
                CURSOR_Y += 1;
            }
        }

        if CURSOR_Y >= VGA_HEIGHT {
            scroll_up();
            CURSOR_Y = VGA_HEIGHT - 1;
        }
        update_cursor();
    }
}

pub fn print_str(s: &str) {
    for byte in s.bytes() {
        print_char(byte);
    }
}

unsafe fn scroll_up() {
    for y in 1..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            let from = (y * VGA_WIDTH + x) * 2;
            let to = ((y - 1) * VGA_WIDTH + x) * 2;
            *VGA_BUFFER.add(to) = *VGA_BUFFER.add(from);
            *VGA_BUFFER.add(to + 1) = *VGA_BUFFER.add(from + 1);
        }
    }
    for x in 0..VGA_WIDTH {
        let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + x) * 2;
        *VGA_BUFFER.add(offset) = b' ';
        *VGA_BUFFER.add(offset + 1) = COLOR;
    }
}

// Magie: Rekne graficke karte, kam ma presunout ten blikajici kurzor
fn update_cursor() {
    unsafe {
        let pos = CURSOR_Y * VGA_WIDTH + CURSOR_X;
        asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Fu8, options(nomem, nostack, preserves_flags));
        asm!("out dx, al", in("dx") 0x3D5u16, in("al") (pos & 0xFF) as u8, options(nomem, nostack, preserves_flags));
        asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Eu8, options(nomem, nostack, preserves_flags));
        asm!("out dx, al", in("dx") 0x3D5u16, in("al") ((pos >> 8) & 0xFF) as u8, options(nomem, nostack, preserves_flags));
    }
}
