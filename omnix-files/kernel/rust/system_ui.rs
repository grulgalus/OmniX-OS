use crate::vga;
use crate::mouse;

static mut TERMINAL_OPEN: bool = false;
static mut SYSINFO_OPEN: bool = false;

pub fn start() {
    mouse::init();

    loop {
        let (mx, my, is_clicked) = mouse::get_state();

        unsafe {
            if is_clicked && mx >= 2 && mx <= 47 && my >= 187 && my <= 198 {
                TERMINAL_OPEN = true;
            }
            if is_clicked && mx >= 10 && mx <= 40 && my >= 50 && my <= 70 {
                SYSINFO_OPEN = true;
            }
            if is_clicked && TERMINAL_OPEN && mx >= 256 && mx <= 268 && my >= 52 && my <= 64 {
                TERMINAL_OPEN = false;
            }
            if is_clicked && SYSINFO_OPEN && mx >= 296 && mx <= 308 && my >= 12 && my <= 24 {
                SYSINFO_OPEN = false;
            }
        }

        // Kreslime VSECHNO jen do neviditelneho modulu (okamzite hotove)
        draw_desktop();

        unsafe {
            if SYSINFO_OPEN { draw_sysinfo(); }
            if TERMINAL_OPEN { draw_terminal(); }
        }

        draw_cursor(mx, my);

        // Nyni vse vysleme NAJEDNOU na obrazovku
        vga::swap_buffers();
        
        // ZADNY FAKE DELAY TADY NENI - JEDEME NA MAXIMALNI FPS
    }
}

fn draw_cursor(x: usize, y: usize) {
    vga::draw_rect(x, y, 2, 6, 15);
    vga::draw_rect(x, y, 5, 2, 15);
    vga::draw_rect(x + 2, y + 2, 2, 2, 15);
}

fn draw_desktop() {
    vga::clear_screen(3); 
    draw_icon(10, 10, b"DISK");
    draw_icon(10, 50, b"SYS");
    
    draw_raised_rect(0, 185, 320, 15, 7); 
    draw_raised_rect(2, 187, 45, 11, 10);
    vga::draw_str(b"START", 8, 189, 0);

    draw_sunken_rect(275, 187, 42, 11, 7);
    vga::draw_str(b"12:00", 280, 189, 0);
}

fn draw_icon(x: usize, y: usize, label: &[u8]) {
    draw_raised_rect(x + 4, y, 20, 18, 7);
    vga::draw_rect(x + 8, y + 4, 12, 10, 1);
    vga::draw_rect(x, y + 20, 30, 9, 1);
    vga::draw_str(label, x + 2, y + 21, 15);
}

fn draw_terminal() {
    let x = 60; let y = 50; let w = 210; let h = 110;
    draw_window(x, y, w, h, b"HACKER TERMINAL");
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);
    vga::draw_str(b">> ACCESS GRANTED <<", x + 8, y + 20, 10);
    vga::draw_str(b"C:/> _", x + 8, y + 35, 10);
}

fn draw_sysinfo() {
    let x = 160; let y = 10; let w = 150; let h = 60;
    draw_window(x, y, w, h, b"SYSTEM INFO");
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);
    vga::draw_str(b"NET: ONLINE", x + 8, y + 20, 10);
}

fn draw_window(x: usize, y: usize, w: usize, h: usize, title: &[u8]) {
    draw_raised_rect(x, y, w, h, 7);
    vga::draw_rect(x + 2, y + 2, w - 4, 12, 1);
    vga::draw_str(title, x + 6, y + 4, 15);
    draw_raised_rect(x + w - 14, y + 2, 12, 12, 7); 
    vga::draw_str(b"X", x + w - 11, y + 4, 0);
}

fn draw_raised_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg);
    vga::draw_rect(x, y, w, 1, 15); 
    vga::draw_rect(x, y, 1, h, 15); 
    vga::draw_rect(x + w - 1, y, 1, h, 8); 
    vga::draw_rect(x, y + h - 1, w, 1, 8); 
}

fn draw_sunken_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg);
    vga::draw_rect(x, y, w, 1, 8); 
    vga::draw_rect(x, y, 1, h, 8); 
    vga::draw_rect(x + w - 1, y, 1, h, 15); 
    vga::draw_rect(x, y + h - 1, w, 1, 15); 
}
