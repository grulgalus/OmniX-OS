use crate::vga;
use crate::keyboard;

pub fn start() {
    draw_desktop();
    draw_sysinfo();
    draw_terminal();

    loop {
        let key = keyboard::read_key();
        
        if key == b'1' {
            draw_desktop();
            draw_sysinfo();
            draw_terminal();
        } 
        else if key == b'2' {
            draw_desktop();
            draw_terminal();
            draw_sysinfo();
        } 
        else if key == 27 {
            draw_desktop();
        }
    }
}

fn draw_desktop() {
    vga::clear_screen(3); // Bright Cyan pozadi presne podle obrazku

    // Ikony na plose
    draw_icon(10, 10, b"DISK");
    draw_icon(10, 50, b"TERM");
    draw_icon(10, 90, b"TRASH");

    // Taskbar dole (sedy)
    draw_raised_rect(0, 185, 320, 15, 7);

    // Zelene START tlacitko z obrazku
    draw_raised_rect(2, 187, 45, 11, 10);
    vga::draw_str(b"START", 8, 189, 0);

    // Hodiny a taskbar ikony vpravo
    draw_sunken_rect(275, 187, 42, 11, 7);
    vga::draw_str(b"12:00", 280, 189, 0);
    
    draw_sunken_rect(210, 187, 60, 11, 7);
    vga::draw_str(b"CPU 87%", 215, 189, 0);
}

fn draw_icon(x: usize, y: usize, label: &[u8]) {
    // Pixel-art ikona
    draw_raised_rect(x + 4, y, 20, 18, 7);
    vga::draw_rect(x + 8, y + 4, 12, 10, 1);
    // Text pod ikonou s cernym pozadim jako stare Windows
    vga::draw_rect(x, y + 20, 30, 9, 1);
    vga::draw_str(label, x + 2, y + 21, 15);
}

fn draw_terminal() {
    let x = 60;
    let y = 50;
    let w = 210;
    let h = 110;
    
    draw_window(x, y, w, h, b"HACKER TERMINAL");
    
    // Cerny vnitrek pro hacker styl
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);
    
    // Zelene svitici texty z tveho obrazku
    vga::draw_str(b">> ACCESS GRANTED <<", x + 8, y + 20, 10);
    vga::draw_str(b"INTRUSION PROTOCOL...", x + 8, y + 35, 10);
    vga::draw_str(b"DECRYPTING DATA...", x + 8, y + 45, 10);
    vga::draw_str(b">> DATA BREACH <<", x + 8, y + 60, 12); // Cervena chyba
    vga::draw_str(b"C:/> TRACE ROUTE...", x + 8, y + 75, 10);
    vga::draw_str(b"C:/> _", x + 8, y + 85, 10);
}

fn draw_sysinfo() {
    let x = 160;
    let y = 10;
    let w = 150;
    let h = 60;
    
    draw_window(x, y, w, h, b"SYSTEM INFO");
    
    // Cerny vnitrek
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);
    
    vga::draw_str(b"NET: ONLINE...", x + 8, y + 20, 10);
    vga::draw_str(b"ENCRYPTION: ACTIVE", x + 8, y + 32, 10);
    vga::draw_str(b"STATUS: CONNECTED", x + 8, y + 44, 10);
}

fn draw_window(x: usize, y: usize, w: usize, h: usize, title: &[u8]) {
    // Sedy zaklad okna
    draw_raised_rect(x, y, w, h, 7);
    
    // Modry Title Bar (Gradient imitace pomoci dvou barev)
    vga::draw_rect(x + 2, y + 2, w - 4, 12, 1);
    vga::draw_str(title, x + 6, y + 4, 15);
    
    // Zavírací tlacitko
    draw_raised_rect(x + w - 14, y + 2, 12, 12, 7);
    vga::draw_str(b"X", x + w - 11, y + 4, 0);
}

fn draw_raised_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg);
    vga::draw_rect(x, y, w, 1, 15); // Horni bila linka
    vga::draw_rect(x, y, 1, h, 15); // Leva bila linka
    vga::draw_rect(x + w - 1, y, 1, h, 8); // Prava tmava linka
    vga::draw_rect(x, y + h - 1, w, 1, 8); // Spodni tmava linka
}

fn draw_sunken_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg);
    vga::draw_rect(x, y, w, 1, 8); // Horni tmava linka
    vga::draw_rect(x, y, 1, h, 8); // Leva tmava linka
    vga::draw_rect(x + w - 1, y, 1, h, 15); // Prava bila linka
    vga::draw_rect(x, y + h - 1, w, 1, 15); // Spodni bila linka
}
