use crate::vga;
use crate::keyboard;

pub fn start() {
    draw_desktop();

    loop {
        let key = keyboard::read_key();
        
        if key == b'1' {
            draw_desktop();
            draw_window(40, 20, 220, 130, b"TERMINAL");
            vga::draw_rect(44, 38, 212, 108, 0);
            vga::draw_str(b"OMNIX OS V1.0", 48, 42, 10);
            vga::draw_str(b"C:/>_", 48, 56, 10);
        } 
        else if key == b'2' {
            draw_desktop();
            draw_window(80, 40, 160, 100, b"SYSTEM INFO");
            vga::draw_str(b"OMNIX OS CORE", 95, 60, 0);
            vga::draw_str(b"CPU: ONLINE", 95, 75, 0);
            vga::draw_str(b"RAM: 64MB", 95, 90, 0);
            vga::draw_str(b"DISK: MOUNTED", 95, 105, 0);
        } 
        else if key == 27 { // ESC = Zavre okna
            draw_desktop();
        }
    }
}

fn draw_desktop() {
    vga::clear_screen(3); // Desktop barva: Cyan

    draw_icon(10, 10, b"TERM (1)", 2); 
    draw_icon(10, 50, b"SYS  (2)", 1);

    draw_raised_rect(0, 185, 320, 15, 7);

    draw_raised_rect(2, 187, 50, 11, 7);
    vga::draw_str(b"START", 8, 189, 0);

    draw_sunken_rect(275, 187, 42, 11, 7);
    vga::draw_str(b"12:00", 280, 189, 0);
}

fn draw_icon(x: usize, y: usize, label: &[u8], color: u8) {
    draw_raised_rect(x + 4, y, 24, 20, color);
    vga::draw_rect(x + 8, y + 4, 16, 12, 15);
    vga::draw_str(label, x - 4, y + 24, 15);
}

fn draw_window(x: usize, y: usize, w: usize, h: usize, title: &[u8]) {
    vga::draw_rect(x + 5, y + 5, w, h, 0);
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
