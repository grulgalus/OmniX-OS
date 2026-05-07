use core::arch::asm;

static mut MOUSE_X: i32 = 160;
static mut MOUSE_Y: i32 = 100;

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

unsafe fn mouse_wait(a_type: u8) {
    let mut timeout = 100000;
    if a_type == 0 {
        while timeout > 0 {
            if (inb(0x64) & 1) == 1 { return; }
            timeout -= 1;
        }
    } else {
        while timeout > 0 {
            if (inb(0x64) & 2) == 0 { return; }
            timeout -= 1;
        }
    }
}

unsafe fn mouse_write(a_write: u8) {
    mouse_wait(1);
    outb(0x64, 0xD4);
    mouse_wait(1);
    outb(0x60, a_write);
}

unsafe fn mouse_read() -> u8 {
    mouse_wait(0);
    inb(0x60)
}

pub fn init() {
    unsafe {
        mouse_wait(1);
        outb(0x64, 0xA8); 
        mouse_wait(1);
        outb(0x64, 0x20); 
        mouse_wait(0);
        let status = inb(0x60) | 2;
        mouse_wait(1);
        outb(0x64, 0x60); 
        mouse_wait(1);
        outb(0x60, status);
        mouse_write(0xF6); 
        mouse_read();
        mouse_write(0xF4); 
        mouse_read();
    }
}

pub fn get_state() -> (usize, usize, bool) {
    unsafe {
        if (inb(0x64) & 1) == 1 {
            let status = inb(0x60);
            let dx = inb(0x60) as i8 as i32;
            let dy = inb(0x60) as i8 as i32;

            if (status & 0x08) != 0 {
                MOUSE_X = (MOUSE_X + dx).clamp(0, 318);
                MOUSE_Y = (MOUSE_Y - dy).clamp(0, 198);
            }
            
            // Vrátíme X, Y a true pokud je levé tlačítko stisknuté
            return (MOUSE_X as usize, MOUSE_Y as usize, (status & 1) == 1);
        }
        (MOUSE_X as usize, MOUSE_Y as usize, false)
    }
}
