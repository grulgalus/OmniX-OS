use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::{addr_of, addr_of_mut};

// -- STAVY OKEN A SYSTÉMU --
static mut TERMINAL_OPEN: bool = false;
static mut SYSINFO_OPEN: bool = false;
static mut DISK_OPEN: bool = false;
static mut START_MENU_OPEN: bool = false;

// Proměnné pro Terminál (historie atd. zůstává)
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

    let mut last_click_state = false;

    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        let key = keyboard::read_key();
        unsafe { BLINK_FRAME = BLINK_FRAME.wrapping_add(1); }

        // --- DETEKCE KLIKNUTÍ MYŠÍ ---
        // Reagujeme jen ve chvíli, kdy tlačítko zrovna "stiskneme" (prevence spamu)
        if is_clicked && !last_click_state {
            unsafe {
                // 1. Tlačítko START (levý dolní roh)
                if mx >= 2 && mx <= 47 && my >= 187 && my <= 198 {
                    START_MENU_OPEN = !START_MENU_OPEN;
                } 
                // Pokud klikneš jinam a Start je otevřený, zavře se
                else if START_MENU_OPEN && (mx > 100 || my < 110) {
                    START_MENU_OPEN = false;
                }

                // 2. Položky ve Start Menu (pokud je otevřené)
                if START_MENU_OPEN && mx >= 2 && mx <= 100 {
                    if my >= 130 && my <= 145 { TERMINAL_OPEN = true; START_MENU_OPEN = false; }
                    if my >= 150 && my <= 165 { SYSINFO_OPEN = true; START_MENU_OPEN = false; }
                    // Zde by mohl být shutdown
                }

                // 3. Ikony na ploše
                // DISK
                if mx >= 10 && mx <= 40 && my >= 10 && my <= 40 { DISK_OPEN = true; }
                // SYSINFO (Původní ikona SYS)
                if mx >= 10 && mx <= 40 && my >= 50 && my <= 80 { SYSINFO_OPEN = true; }

                // 4. Křížky pro zavření oken
                if TERMINAL_OPEN && mx >= 256 && mx <= 268 && my >= 52 && my <= 64 { TERMINAL_OPEN = false; }
                if SYSINFO_OPEN && mx >= 296 && mx <= 308 && my >= 12 && my <= 24 { SYSINFO_OPEN = false; }
                if DISK_OPEN && mx >= 256 && mx <= 268 && my >= 82 && my <= 94 { DISK_OPEN = false; }
            }
        }
        last_click_state = is_clicked;

        // --- ZPRACOVÁNÍ KLÁVESNICE ---
        unsafe {
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

        // --- VYKRESLENÍ (odspodu nahoru) ---
        draw_desktop();

        unsafe {
            // Správce oken nakreslí jen to, co má být vidět!
            if DISK_OPEN { draw_disk_manager(); }
            if SYSINFO_OPEN { draw_sysinfo(); }
            if TERMINAL_OPEN { draw_terminal(); }
            if START_MENU_OPEN { draw_start_menu(); }
        }

        draw_cursor(mx, my);
        vga::swap_buffers();
    }
}

// --- VYKRESLOVACÍ FUNKCE ---

fn draw_start_menu() {
    let y = 110;
    // Tělo menu
    draw_raised_rect(2, y, 100, 75, 7);
    // Levý modrý pruh
    vga::draw_rect(4, y + 2, 20, 71, 1);
    vga::draw_str(b"OMNIX", 8, y + 60, 15); // Nápis dole
    
    // Položky
    vga::draw_str(b"TERMINAL", 30, y + 20, 0);
    vga::draw_str(b"SYS INFO", 30, y + 40, 0);
    
    vga::draw_rect(25, y + 60, 70, 1, 8); // Čára oddělující restart
    vga::draw_str(b"REBOOT", 30, y + 65, 0);
}

fn draw_disk_manager() {
    let x = 60; let y = 80; let w = 210; let h = 100;
    draw_window(x, y, w, h, b"FILE EXPLORER");
    draw_sunken_rect(x + 4, y + 16, w - 8, h - 20, 15); // Bílé pozadí

    // "Fake" výpis souborů
    vga::draw_str(b"C:/OMNIX/", x + 8, y + 20, 8);
    vga::draw_str(b" [DIR] KERNEL", x + 8, y + 35, 0);
    vga::draw_str(b" [DIR] APPS", x + 8, y + 45, 0);
    vga::draw_str(b" 42KB  BOOT.BIN", x + 8, y + 55, 0);
    vga::draw_str(b"  1MB  APP.OMXAPK", x + 8, y + 65, 0);
}

// Zbytek logiky z minula (process_command, history, desktop, atd.) zůstává stejný:

unsafe fn process_command() {
    let buf_ptr = addr_of!(TERM_BUF) as *const u8;
    let buf_slice = core::slice::from_raw_parts(buf_ptr, TERM_LEN);
    push_history(buf_slice, TERM_LEN);

    let is_help = TERM_LEN == 4 && *buf_ptr.add(0) == b'H' && *buf_ptr.add(1) == b'E' && *buf_ptr.add(2) == b'L' && *buf_ptr.add(3) == b'P';
    let is_cls = TERM_LEN == 3 && *buf_ptr.add(0) == b'C' && *buf_ptr.add(1) == b'L' && *buf_ptr.add(2) == b'S';
    let is_run = TERM_LEN == 3 && *buf_ptr.add(0) == b'R' && *buf_ptr.add(1) == b'U' && *buf_ptr.add(2) == b'N';
    
    if is_help { push_history(b"CMDS: HELP, CLS, RUN", 20); } 
    else if is_cls { let h_len_ptr = addr_of_mut!(TERM_HIST_LEN); for i in 0..6 { (*h_len_ptr)[i] = 0; } } 
    else if is_run { push_history(b"LAUNCHING APP...", 16); crate::vga::swap_buffers(); crate::omxapk::run_app(200); } 
    else if TERM_LEN > 0 { push_history(b"BAD COMMAND!", 12); }
    TERM_LEN = 0; 
}

unsafe fn push_history(text: &[u8], len: usize) {
    let hist_ptr = addr_of_mut!(TERM_HIST) as *mut [u8; 22];
    let lens_ptr = addr_of_mut!(TERM_HIST_LEN) as *mut usize;
    for i in 0..5 {
        for j in 0..22 { let val = (*hist_ptr.add(i + 1))[j]; (*hist_ptr.add(i))[j] = val; }
        *lens_ptr.add(i) = *lens_ptr.add(i + 1);
    }
    let l = if len > 22 { 22 } else { len };
    let text_ptr = text.as_ptr();
    for i in 0..l { (*hist_ptr.add(5))[i] = *text_ptr.add(i); }
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
                let text_ptr = hist_ptr.add(i) as *const u8;
                let text = core::slice::from_raw_parts(text_ptr, len);
                vga::draw_str(text, x + 8, y + 20 + (i * 10), 10);
            }
        }
        vga::draw_str(b">", x + 8, y + 84, 10);
        if TERM_LEN > 0 { 
            let buf_ptr = addr_of!(TERM_BUF) as *const u8;
            let current_text = core::slice::from_raw_parts(buf_ptr, TERM_LEN);
            vga::draw_str(current_text, x + 18, y + 84, 10); 
        }
        if (BLINK_FRAME % 60) < 30 { vga::draw_rect(x + 18 + (TERM_LEN * 8), y + 84, 6, 8, 10); }
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
