use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::{addr_of, addr_of_mut};

static mut OS_STATE: u8 = 0; 
static mut IS_ADMIN: bool = false;
static mut BG_COLOR: u8 = 3; 

static mut NOTIF_MSG: &'static [u8] = b"";
static mut NOTIF_TIMER: u16 = 0;

static mut TERM_BUF: [u8; 22] = [0; 22]; 
static mut TERM_LEN: usize = 0;          
static mut TERM_HIST: [[u8; 22]; 6] = [[0; 22]; 6]; 
static mut TERM_HIST_LEN: [usize; 6] = [0; 6];
static mut NOTES_BUF: [u8; 60] = [0; 60];
static mut NOTES_LEN: usize = 0;

#[derive(Copy, Clone)]
struct Window { id: u8, x: usize, y: usize, w: usize, h: usize, title: &'static [u8], visible: bool }

static mut WINDOWS: [Window; 4] = [
    Window { id: 1, x: 60, y: 50, w: 210, h: 110, title: b"TERMINAL", visible: false },
    Window { id: 2, x: 40, y: 30, w: 200, h: 100, title: b"FILE EXPLORER", visible: false },
    Window { id: 3, x: 80, y: 60, w: 180, h: 100, title: b"NOTES", visible: false },
    Window { id: 4, x: 100, y: 40, w: 160, h: 110, title: b"SETTINGS", visible: false },
];
static mut ACTIVE_WIN: usize = 0; 
static mut START_MENU_OPEN: bool = false;
static mut BLINK_FRAME: u32 = 0;         

pub fn start() {
    mouse::init();
    show_notif(b"SYSTEM BOOTED");

    let mut last_click = false;
    let mut last_key = 0;

    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        let key = keyboard::read_key();
        unsafe { 
            BLINK_FRAME = BLINK_FRAME.wrapping_add(1); 
            let timer_ptr = addr_of_mut!(NOTIF_TIMER);
            if *timer_ptr > 0 { *timer_ptr -= 1; }
        }

        let clicked_now = is_clicked && !last_click;
        let key_pressed_now = key != 0 && key != last_key;
        
        unsafe {
            if OS_STATE == 0 {
                draw_login(mx, my);
                if clicked_now {
                    if mx > 110 && mx < 210 && my > 80 && my < 100 {
                        IS_ADMIN = true; OS_STATE = 1; show_notif(b"WELCOME ADMIN");
                    }
                    if mx > 110 && mx < 210 && my > 110 && my < 130 {
                        IS_ADMIN = false; OS_STATE = 1; show_notif(b"WELCOME USER");
                    }
                }
            } else {
                handle_desktop_clicks(mx, my, clicked_now);
                handle_keyboard(key, key_pressed_now);
                draw_desktop();
                draw_windows();
                if START_MENU_OPEN { draw_start_menu(); }
                draw_notifications();
            }
        }
        
        last_click = is_clicked;
        last_key = key;
        draw_cursor(mx, my);
        vga::swap_buffers();
    }
}

fn show_notif(msg: &'static [u8]) {
    unsafe {
        *addr_of_mut!(NOTIF_MSG) = msg;
        *addr_of_mut!(NOTIF_TIMER) = 200; 
    }
}

unsafe fn handle_desktop_clicks(mx: usize, my: usize, clicked: bool) {
    if !clicked { return; }

    if mx >= 2 && mx <= 47 && my >= 187 && my <= 198 { START_MENU_OPEN = !START_MENU_OPEN; return; }
    if START_MENU_OPEN {
        if mx < 100 {
            if my > 90 && my < 105 { toggle_window(1); } 
            if my > 110 && my < 125 { toggle_window(2); } 
            if my > 130 && my < 145 { toggle_window(3); } 
            if my > 150 && my < 165 { toggle_window(4); } 
            if my > 170 && my < 185 { OS_STATE = 0; show_notif(b"LOGGED OUT"); } 
        }
        START_MENU_OPEN = false;
        return;
    }

    if mx >= 10 && mx <= 40 && my >= 10 && my <= 40 { toggle_window(2); } 
    if mx >= 10 && mx <= 40 && my >= 50 && my <= 80 { toggle_window(4); } 

    let wins = addr_of_mut!(WINDOWS) as *mut [Window; 4];
    for i in (0..4).rev() { 
        let w = &mut (*wins)[i];
        if w.visible && mx >= w.x && mx <= w.x + w.w && my >= w.y && my <= w.y + w.h {
            ACTIVE_WIN = i; 
            
            if mx >= w.x + w.w - 14 && my <= w.y + 14 { w.visible = false; }
            
            if w.id == 4 {
                if mx > w.x + 10 && mx < w.x + 50 && my > w.y + 40 && my < w.y + 55 { *addr_of_mut!(BG_COLOR) = 3; show_notif(b"WP: CYAN"); }
                if mx > w.x + 60 && mx < w.x + 100 && my > w.y + 40 && my < w.y + 55 { *addr_of_mut!(BG_COLOR) = 1; show_notif(b"WP: BLUE"); }
                if mx > w.x + 110 && mx < w.x + 150 && my > w.y + 40 && my < w.y + 55 { *addr_of_mut!(BG_COLOR) = 2; show_notif(b"WP: GREEN"); }
            }
            return;
        }
    }
}

unsafe fn toggle_window(id: u8) {
    let wins = addr_of_mut!(WINDOWS) as *mut [Window; 4];
    for i in 0..4 {
        if (*wins)[i].id == id {
            (*wins)[i].visible = true;
            ACTIVE_WIN = i;
        }
    }
}

unsafe fn handle_keyboard(key: u8, pressed: bool) {
    if !pressed || key == 0 { return; }

    if key == 9 {
        let wins = addr_of_mut!(WINDOWS) as *mut [Window; 4];
        for _ in 0..4 {
            ACTIVE_WIN = (ACTIVE_WIN + 1) % 4;
            if (*wins)[ACTIVE_WIN].visible { break; }
        }
        return;
    }

    let wins = addr_of!(WINDOWS) as *const [Window; 4];
    let active_id = (*wins)[ACTIVE_WIN].id;
    let is_visible = (*wins)[ACTIVE_WIN].visible;

    if !is_visible { return; }

    if active_id == 1 { 
        if key == 8 { if TERM_LEN > 0 { TERM_LEN -= 1; } } 
        else if key == b'\n' { process_command(); } 
        else if key >= 32 && key <= 126 && TERM_LEN < 22 {
            let mut k = key; if k >= b'a' && k <= b'z' { k -= 32; }
            let ptr = addr_of_mut!(TERM_BUF) as *mut u8; ptr.add(TERM_LEN).write(k); TERM_LEN += 1;
        }
    } 
    else if active_id == 3 { 
        if key == 8 { if NOTES_LEN > 0 { NOTES_LEN -= 1; } } 
        else if key >= 32 && key <= 126 && NOTES_LEN < 60 {
            let ptr = addr_of_mut!(NOTES_BUF) as *mut u8; ptr.add(NOTES_LEN).write(key); NOTES_LEN += 1;
        }
    }
}

fn draw_login(mx: usize, my: usize) {
    vga::clear_screen(1); 
    draw_window_frame(100, 50, 120, 100, b"LOGIN", true);
    
    let admin_c = if mx > 110 && mx < 210 && my > 80 && my < 100 { 10 } else { 7 };
    draw_raised_rect(110, 80, 100, 20, admin_c);
    vga::draw_str(b"ADMIN", 140, 86, 0);

    let user_c = if mx > 110 && mx < 210 && my > 110 && my < 130 { 10 } else { 7 };
    draw_raised_rect(110, 110, 100, 20, user_c);
    vga::draw_str(b"USER", 145, 116, 0);
}

fn draw_desktop() {
    vga::clear_screen(unsafe { BG_COLOR }); 
    draw_icon(10, 10, b"DISK"); draw_icon(10, 50, b"SYS");
    
    draw_raised_rect(0, 185, 320, 15, 7); 
    draw_raised_rect(2, 187, 45, 11, 10); 
    vga::draw_str(b"START", 8, 189, 0);
    
    draw_sunken_rect(275, 187, 42, 11, 7); 
    let time = unsafe { crate::rtc::get_time() };
    vga::draw_str(time, 280, 189, 0);
}

fn draw_start_menu() {
    draw_raised_rect(2, 85, 100, 100, 7);
    vga::draw_rect(4, 87, 15, 96, 1);
    vga::draw_str(b"OS", 6, 160, 15); 
    
    vga::draw_str(b"TERMINAL", 25, 95, 0);
    vga::draw_str(b"FILES", 25, 115, 0);
    vga::draw_str(b"NOTES", 25, 135, 0);
    vga::draw_str(b"SETTINGS", 25, 155, 0);
    vga::draw_rect(25, 170, 70, 1, 8); 
    vga::draw_str(b"LOGOUT", 25, 175, 0);
}

unsafe fn draw_windows() {
    let wins = addr_of!(WINDOWS) as *const [Window; 4];
    for i in 0..4 {
        if i != ACTIVE_WIN && (*wins)[i].visible { draw_app_window(&(*wins)[i], false); }
    }
    if (*wins)[ACTIVE_WIN].visible { draw_app_window(&(*wins)[ACTIVE_WIN], true); }
}

unsafe fn draw_app_window(w: &Window, is_active: bool) {
    vga::draw_rect(w.x + 2, w.y + 2, w.w, w.h, 0);
    draw_window_frame(w.x, w.y, w.w, w.h, w.title, is_active);

    if w.id == 1 { 
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 0);
        let h_len_ptr = addr_of!(TERM_HIST_LEN) as *const usize;
        let h_ptr = addr_of!(TERM_HIST) as *const [u8; 22];
        for i in 0..6 {
            let len = *h_len_ptr.add(i);
            if len > 0 {
                let text = core::slice::from_raw_parts(h_ptr.add(i) as *const u8, len);
                vga::draw_str(text, w.x + 8, w.y + 20 + (i * 10), 10);
            }
        }
        vga::draw_str(b">", w.x + 8, w.y + 84, 10);
        if TERM_LEN > 0 { 
            let text = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
            vga::draw_str(text, w.x + 18, w.y + 84, 10); 
        }
        if is_active && (BLINK_FRAME % 60) < 30 { vga::draw_rect(w.x + 18 + (TERM_LEN * 8), w.y + 84, 6, 8, 10); }
    } 
    else if w.id == 2 { 
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 15);
        if IS_ADMIN { vga::draw_str(b"C:/ ROOT ACCESS", w.x + 8, w.y + 20, 12); } 
        else { vga::draw_str(b"C:/ USER", w.x + 8, w.y + 20, 8); }
        vga::draw_str(b"[DIR] SYSTEM", w.x + 8, w.y + 35, 0);
        vga::draw_str(b"[DIR] APPS", w.x + 8, w.y + 45, 0);
        vga::draw_str(b"1.4MB APP.OMXAPK", w.x + 8, w.y + 55, 0);
    }
    else if w.id == 3 { 
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 15);
        if NOTES_LEN > 0 {
            let text = core::slice::from_raw_parts(addr_of!(NOTES_BUF) as *const u8, NOTES_LEN);
            let mut ty = w.y + 20; let mut tx = w.x + 8;
            for &c in text {
                vga::draw_char(c, tx, ty, 0); tx += 8;
                if tx > w.x + w.w - 16 { tx = w.x + 8; ty += 10; }
            }
            if is_active && (BLINK_FRAME % 60) < 30 { vga::draw_rect(tx, ty, 6, 8, 0); }
        } else if is_active && (BLINK_FRAME % 60) < 30 { vga::draw_rect(w.x + 8, w.y + 20, 6, 8, 0); }
    }
    else if w.id == 4 { 
        vga::draw_str(b"WALLPAPER COLOR:", w.x + 10, w.y + 25, 0);
        draw_raised_rect(w.x + 10, w.y + 40, 40, 15, 3); 
        draw_raised_rect(w.x + 60, w.y + 40, 40, 15, 1); 
        draw_raised_rect(w.x + 110, w.y + 40, 40, 15, 2); 
        
        vga::draw_str(b"BRIGHTNESS [||||  ]", w.x + 10, w.y + 70, 8);
        vga::draw_str(b"SOUND      [|||   ]", w.x + 10, w.y + 85, 8);
    }
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

fn draw_window_frame(x: usize, y: usize, w: usize, h: usize, title: &[u8], is_active: bool) {
    draw_raised_rect(x, y, w, h, 7); 
    let title_c = if is_active { 1 } else { 8 };
    vga::draw_rect(x + 2, y + 2, w - 4, 12, title_c);
    vga::draw_str(title, x + 6, y + 4, 15); 
    draw_raised_rect(x + w - 14, y + 2, 12, 12, 7); vga::draw_str(b"X", x + w - 11, y + 4, 0);
}

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

unsafe fn process_command() {
    let buf_ptr = addr_of!(TERM_BUF) as *const u8;
    let buf_slice = core::slice::from_raw_parts(buf_ptr, TERM_LEN);
    push_history(buf_slice, TERM_LEN);

    let is_help = TERM_LEN == 4 && *buf_ptr.add(0) == b'H' && *buf_ptr.add(1) == b'E' && *buf_ptr.add(2) == b'L' && *buf_ptr.add(3) == b'P';
    let is_run = TERM_LEN == 3 && *buf_ptr.add(0) == b'R' && *buf_ptr.add(1) == b'U' && *buf_ptr.add(2) == b'N';
    
    if is_help { push_history(b"CMDS: HELP, RUN", 15); } 
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
