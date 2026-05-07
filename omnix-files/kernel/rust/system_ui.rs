use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::{addr_of, addr_of_mut};

static mut TERMINAL_OPEN: bool = false;
static mut SYSINFO_OPEN: bool = false;

static mut TERM_BUF: [u8; 22] = [0; 22]; 
static mut TERM_LEN: usize = 0;          
static mut TERM_HIST: [[u8; 22]; 6] = [[0; 22]; 6]; 
static mut TERM_HIST_LEN: [usize; 6] = [0; 6];
static mut BLINK_FRAME: u32 = 0;         

pub fn start() {
    mouse::init();

    unsafe {
        push_history(b"OMNIX OS TERMINAL V1", 20);
        push_history(b"TYPE 'HELP' FOR CMDS", 20);
    }

    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        // Oprava warningu: Odebrano "mut" protoze "key" uz nemodifikujeme
        let key = keyboard::read_key();

        unsafe {
            BLINK_FRAME = BLINK_FRAME.wrapping_add(1);

            if is_clicked && mx >= 2 && mx <= 47 && my >= 187 && my <= 198 { TERMINAL_OPEN = true; }
            if is_clicked && mx >= 10 && mx <= 40 && my >= 50 && my <= 70 { SYSINFO_OPEN = true; }
            if is_clicked && TERMINAL_OPEN && mx >= 256 && mx <= 268 && my >= 52 && my <= 64 { TERMINAL_OPEN = false; }
            if is_clicked && SYSINFO_OPEN && mx >= 296 && mx <= 308 && my >= 12 && my <= 24 { SYSINFO_OPEN = false; }

            if TERMINAL_OPEN && key != 0 {
                if key == 8 { 
                    if TERM_LEN > 0 { TERM_LEN -= 1; }
                } else if key == b'\n' { 
                    process_command();
                } else if key >= 32 && key <= 126 && TERM_LEN < 22 {
                    let mut k = key;
                    if k >= b'a' && k <= b'z' { k -= 32; }
                    let ptr = addr_of_mut!(TERM_BUF) as *mut u8;
                    ptr.add(TERM_LEN).write(k);
                    TERM_LEN += 1;
                }
            }
        }

        draw_desktop();

        unsafe {
            if SYSINFO_OPEN { draw_sysinfo(); }
            if TERMINAL_OPEN { draw_terminal(); }
        }

        draw_cursor(mx, my);
        vga::swap_buffers();
    }
}

// ZPRACOVANI PRIKAZU V TERMINALU
unsafe fn process_command() {
    // 1. TADY JE TA OPRAVA CHYBY! Musime definovat buf_ptr jako prvni vec.
    let buf_ptr = addr_of!(TERM_BUF) as *const u8;
    
    // Zapiseme radek do historie
    let buf_slice = core::slice::from_raw_parts(buf_ptr, TERM_LEN);
    push_history(buf_slice, TERM_LEN);

    // 2. Definice jednotlivych prikazu (nyni uz maji buf_ptr k dispozici)
    let is_help = TERM_LEN == 4 && *buf_ptr.add(0) == b'H' && *buf_ptr.add(1) == b'E' && *buf_ptr.add(2) == b'L' && *buf_ptr.add(3) == b'P';
    let is_cls = TERM_LEN == 3 && *buf_ptr.add(0) == b'C' && *buf_ptr.add(1) == b'L' && *buf_ptr.add(2) == b'S';
    let is_ver = TERM_LEN == 3 && *buf_ptr.add(0) == b'V' && *buf_ptr.add(1) == b'E' && *buf_ptr.add(2) == b'R';
    let is_run = TERM_LEN == 3 && *buf_ptr.add(0) == b'R' && *buf_ptr.add(1) == b'U' && *buf_ptr.add(2) == b'N';
    let is_time = TERM_LEN == 4 && *buf_ptr.add(0) == b'T' && *buf_ptr.add(1) == b'I' && *buf_ptr.add(2) == b'M' && *buf_ptr.add(3) == b'E';
    let is_whoami = TERM_LEN == 6 && *buf_ptr.add(0) == b'W' && *buf_ptr.add(1) == b'H' && *buf_ptr.add(2) == b'O' && *buf_ptr.add(3) == b'A' && *buf_ptr.add(4) == b'M' && *buf_ptr.add(5) == b'I';

    // 3. Provedeni logiky
    if is_help {
        push_history(b"CMDS: HELP, CLS, VER, RUN", 20);
        push_history(b"TIME, WHOAMI", 12); // Pridano na druhy radek, at se to vejde
    } else if is_cls {
        let h_len_ptr = addr_of_mut!(TERM_HIST_LEN);
        for i in 0..6 { (*h_len_ptr)[i] = 0; } 
    } else if is_ver {
        push_history(b"OMNIX OS CORE 1.0", 17);
    } else if is_run {
        push_history(b"LAUNCHING APP...", 16);
        crate::vga::swap_buffers();
        crate::omxapk::run_app(200); 
    } else if is_time {
        let time_str = crate::rtc::get_time();
        push_history(b"SYS TIME IS:", 12);
        push_history(time_str, 5);
    } else if is_whoami {
        push_history(b"ROOT / ADMIN", 12);
    } else if TERM_LEN > 0 {
        push_history(b"BAD COMMAND!", 12);
    }
    
    TERM_LEN = 0; 
}

unsafe fn push_history(text: &[u8], len: usize) {
    let hist_ptr = addr_of_mut!(TERM_HIST) as *mut [u8; 22];
    let lens_ptr = addr_of_mut!(TERM_HIST_LEN) as *mut usize;

    for i in 0..5 {
        for j in 0..22 {
            let val = (*hist_ptr.add(i + 1))[j];
            (*hist_ptr.add(i))[j] = val;
        }
        *lens_ptr.add(i) = *lens_ptr.add(i + 1);
    }

    let l = if len > 22 { 22 } else { len };
    let text_ptr = text.as_ptr();

    for i in 0..l {
        (*hist_ptr.add(5))[i] = *text_ptr.add(i);
    }

    *lens_ptr.add(5) = l;
}

fn draw_cursor(x: usize, y: usize) {
    vga::draw_rect(x, y, 2, 6, 15); vga::draw_rect(x, y, 5, 2, 15); vga::draw_rect(x + 2, y + 2, 2, 2, 15);
}

fn draw_desktop() {
    vga::clear_screen(3); 
    draw_icon(10, 10, b"DISK"); draw_icon(10, 50, b"SYS");
    draw_raised_rect(0, 185, 320, 15, 7); 
    draw_raised_rect(2, 187, 45, 11, 10);
    vga::draw_str(b"START", 8, 189, 0);
    draw_sunken_rect(275, 187, 42, 11, 7); 
    
    // Zobrazuje skutecny cas pres RTC modul!
    let time = unsafe { crate::rtc::get_time() };
    vga::draw_str(time, 280, 189, 0);
}

fn draw_icon(x: usize, y: usize, label: &[u8]) {
    draw_raised_rect(x + 4, y, 20, 18, 7); vga::draw_rect(x + 8, y + 4, 12, 10, 1);
    vga::draw_rect(x, y + 20, 30, 9, 1); vga::draw_str(label, x + 2, y + 21, 15);
}

fn draw_terminal() {
    let x = 60; let y = 50; let w = 210; let h = 110;
    draw_window(x, y, w, h, b"HACKER TERMINAL");
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);

    unsafe {
        let hist_len_ptr = addr_of!(TERM_HIST_LEN) as *const usize;
        let hist_ptr = addr_of!(TERM_HIST) as *const [u8; 22];
        for i in 0..6 {
            let len = *hist_len_ptr.add(i);
            if len > 0 {
                // OPRAVA ZDE:
                // Ziskame adresu daneho radku a rovnou ji pretypujeme na ukazatel na pismenka
                let text_ptr = hist_ptr.add(i) as *const u8;
                let text = core::slice::from_raw_parts(text_ptr, len);
                
                vga::draw_str(text, x + 8, y + 20 + (i * 10), 10);
            }
        }
        if (BLINK_FRAME % 60) < 30 { vga::draw_rect(x + 18 + (TERM_LEN * 8), y + 84, 6, 8, 10); }
        vga::draw_str(b">", x + 8, y + 84, 10);
    }
}

fn draw_sysinfo() {
    let x = 160; let y = 10; let w = 150; let h = 60;
    draw_window(x, y, w, h, b"SYSTEM INFO");
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 0);
    vga::draw_str(b"NET: ONLINE", x + 8, y + 20, 10);
    vga::draw_str(b"OS: OMNIX 1.0", x + 8, y + 32, 10);
}

fn draw_window(x: usize, y: usize, w: usize, h: usize, title: &[u8]) {
    draw_raised_rect(x, y, w, h, 7); vga::draw_rect(x + 2, y + 2, w - 4, 12, 1);
    vga::draw_str(title, x + 6, y + 4, 15); draw_raised_rect(x + w - 14, y + 2, 12, 12, 7); vga::draw_str(b"X", x + w - 11, y + 4, 0);
}

fn draw_raised_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg); vga::draw_rect(x, y, w, 1, 15); vga::draw_rect(x, y, 1, h, 15); 
    vga::draw_rect(x + w - 1, y, 1, h, 8); vga::draw_rect(x, y + h - 1, w, 1, 8); 
}

fn draw_sunken_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg); vga::draw_rect(x, y, w, 1, 8); vga::draw_rect(x, y, 1, h, 8); 
    vga::draw_rect(x + w - 1, y, 1, h, 15); vga::draw_rect(x, y + h - 1, w, 1, 15); 
}
