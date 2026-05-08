use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::{addr_of, addr_of_mut};

static mut OS_STATE: u8 = 0; 
static mut IS_ADMIN: bool = false;
static mut BG_COLOR: u8 = 3; 
static mut BOOT_PROGRESS: u32 = 0;

static mut NOTIF_MSG: &'static [u8] = b"";
static mut NOTIF_TIMER: u16 = 0;

// Paměť pro terminál
static mut TERM_BUF: [u8; 30] = [0; 30]; 
static mut TERM_LEN: usize = 0;          
static mut TERM_HIST: [[u8; 30]; 12] = [[0; 30]; 12]; 
static mut TERM_HIST_LEN: [usize; 12] = [0; 12];

const SCREEN_W: usize = 320;
const SCREEN_H: usize = 200;

pub fn start() {
    mouse::init();

    let mut last_click = false;
    let mut last_key = 0;
    
    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        let key = keyboard::read_key();
        
        let clicked_now = is_clicked && !last_click;
        let key_pressed_now = key != 0 && key != last_key;
        
        unsafe {
            let timer_ptr = addr_of_mut!(NOTIF_TIMER);
            if *timer_ptr > 0 { *timer_ptr -= 1; }

            if OS_STATE == 0 {
                // --- 1. BOOT ANIMACE ---
                draw_boot_animation();
                BOOT_PROGRESS += 1;
                if BOOT_PROGRESS > 180 { OS_STATE = 1; }
            } 
            else if OS_STATE == 1 {
                // --- 2. WINDOWS LOGIN ---
                draw_windows_login(mx, my);
                if clicked_now {
                    if mx > 110 && mx < 210 && my > 130 && my < 150 {
                        IS_ADMIN = true; OS_STATE = 2; show_notif(b"ADMIN LOGIN");
                    }
                    if mx > 110 && mx < 210 && my > 160 && my < 180 {
                        IS_ADMIN = false; OS_STATE = 2; show_notif(b"GUEST LOGIN");
                    }
                }
            } 
            else {
                // --- 3. DESKTOP A TERMINÁL ---
                draw_desktop();
                draw_terminal(mx, my);
                handle_keyboard(key, key_pressed_now);
                draw_notifications();
            }
        }
        
        last_click = is_clicked;
        last_key = key;
        if unsafe { OS_STATE } > 0 { draw_cursor(mx, my); }
        vga::swap_buffers();
    }
}

// ------------------------------
// --- BOOT A LOGIN (320x200) ---
// ------------------------------

unsafe fn draw_boot_animation() {
    vga::clear_screen(0); 
    vga::draw_str(b"OMNIX OS", 120, 80, 15);
    vga::draw_str(b"Starting...", 110, 100, 7);

    draw_sunken_rect(60, 120, 200, 15, 0); 
    let bar_width = if BOOT_PROGRESS > 196 { 196 } else { BOOT_PROGRESS as usize };
    vga::draw_rect(62, 122, bar_width, 11, 2); 
}

fn draw_windows_login(mx: usize, my: usize) {
    vga::clear_screen(1); 
    
    draw_raised_rect(130, 40, 60, 60, 7);
    vga::draw_rect(145, 50, 30, 30, 15); 
    vga::draw_rect(135, 85, 50, 15, 15); 
    vga::draw_str(b"OMNIX", 140, 110, 15);

    let admin_c = if mx > 110 && mx < 210 && my > 130 && my < 150 { 10 } else { 7 };
    draw_raised_rect(110, 130, 100, 20, admin_c);
    vga::draw_str(b"ADMIN", 140, 136, 0);

    let user_c = if mx > 110 && mx < 210 && my > 160 && my < 180 { 10 } else { 7 };
    draw_raised_rect(110, 160, 100, 20, user_c);
    vga::draw_str(b"USER", 145, 166, 0);
}

// ------------------------------
// --- DESKTOP A UI ---
// ------------------------------

fn draw_desktop() {
    vga::clear_screen(unsafe { BG_COLOR });
    draw_icon(10, 10, b"DISK"); draw_icon(10, 50, b"SYS");
    
    // Spodní lišta
    draw_raised_rect(0, 180, 320, 20, 7); 
    draw_raised_rect(2, 182, 45, 16, 10);
    vga::draw_str(b"START", 8, 186, 0);
}

unsafe fn draw_terminal(mx: usize, my: usize) {
    let wx = 60; let wy = 20; let ww = 240; let wh = 140;
    
    // Okno
    draw_raised_rect(wx, wy, ww, wh, 7); 
    vga::draw_rect(wx + 2, wy + 2, ww - 4, 12, 1);
    vga::draw_str(b"TERMINAL", wx + 6, wy + 4, 15); 
    
    // Vnitřek terminálu
    draw_sunken_rect(wx + 4, wy + 16, ww - 8, wh - 20, 0);
    
    let h_len_ptr = addr_of!(TERM_HIST_LEN) as *const usize;
    let h_ptr = addr_of!(TERM_HIST) as *const [u8; 30];
    
    for i in 0..11 {
        let len = *h_len_ptr.add(i);
        if len > 0 {
            let text = core::slice::from_raw_parts(h_ptr.add(i) as *const u8, len);
            vga::draw_str(text, wx + 8, wy + 20 + (i * 10), 10);
        }
    }
    
    let input_y = wy + 130; 
    vga::draw_str(b">", wx + 8, input_y, 10);
    
    if TERM_LEN > 0 { 
        let text = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
        vga::draw_str(text, wx + 18, input_y, 10); 
    }
}

unsafe fn handle_keyboard(key: u8, pressed: bool) {
    if !pressed || key == 0 { return; }

    if key == 8 { if TERM_LEN > 0 { TERM_LEN -= 1; } } 
    else if key == b'\n' { 
        // Pokud stiskne Enter, ulož do historie
        let buf_slice = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
        push_history(buf_slice, TERM_LEN);
        TERM_LEN = 0; 
    } 
    else if key >= 32 && key <= 126 && TERM_LEN < 25 {
        let ptr = addr_of_mut!(TERM_BUF) as *mut u8; ptr.add(TERM_LEN).write(key); TERM_LEN += 1;
    }
}

unsafe fn push_history(text: &[u8], len: usize) {
    let hist_ptr = addr_of_mut!(TERM_HIST) as *mut [u8; 30];
    let lens_ptr = addr_of_mut!(TERM_HIST_LEN) as *mut usize;
    
    for i in 0..10 {
        let src = hist_ptr.add(i + 1) as *const u8;
        let dst = hist_ptr.add(i) as *mut u8;
        for j in 0..30 { *dst.add(j) = *src.add(j); }
        *lens_ptr.add(i) = *lens_ptr.add(i + 1);
    }
    
    let l = if len > 30 { 30 } else { len };
    let text_ptr = text.as_ptr();
    let dst = hist_ptr.add(10) as *mut u8;
    for i in 0..l { *dst.add(i) = *text_ptr.add(i); }
    *lens_ptr.add(10) = l;
}

fn show_notif(msg: &'static [u8]) {
    unsafe { *addr_of_mut!(NOTIF_MSG) = msg; *addr_of_mut!(NOTIF_TIMER) = 200; }
}

fn draw_notifications() {
    unsafe {
        let timer = *addr_of!(NOTIF_TIMER);
        if timer > 0 {
            let msg = *addr_of!(NOTIF_MSG);
            let w = (msg.len() * 8) + 16;
            draw_raised_rect(310 - w, 5, w, 20, 14); 
            vga::draw_str(msg, 318 - w, 11, 0);
        }
    }
}

// --- POMOCNÉ KRESLÍCÍ FUNKCE ---
fn draw_icon(x: usize, y: usize, label: &[u8]) {
    draw_raised_rect(x + 4, y, 20, 18, 7); vga::draw_rect(x + 8, y + 4, 12, 10, 1);
    vga::draw_rect(x, y + 20, 30, 9, 1); vga::draw_str(label, x + 2, y + 21, 15);
}
fn draw_cursor(x: usize, y: usize) {
    vga::draw_rect(x, y, 2, 6, 15); vga::draw_rect(x, y, 5, 2, 15); vga::draw_rect(x + 2, y + 2, 2, 2, 15);
}
fn draw_raised_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg); vga::draw_rect(x, y, w, 1, 15); vga::draw_rect(x, y, 1, h, 15); 
    vga::draw_rect(x + w - 1, y, 1, h, 8); vga::draw_rect(x, y + h - 1, w, 1, 8); 
}
fn draw_sunken_rect(x: usize, y: usize, w: usize, h: usize, bg: u8) {
    vga::draw_rect(x, y, w, h, bg); vga::draw_rect(x, y, w, 1, 8); vga::draw_rect(x, y, 1, h, 8); 
    vga::draw_rect(x + w - 1, y, 1, h, 15); vga::draw_rect(x, y + h - 1, w, 1, 15); 
}
