use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::{addr_of, addr_of_mut};

static mut BG_COLOR: u8 = 3;
static mut START_MENU_OPEN: bool = false;
static mut BLINK_FRAME: u32 = 0;

static mut TERM_BUF: [u8; 30] = [0; 30];
static mut TERM_LEN: usize = 0;
static mut TERM_HIST: [[u8; 30]; 12] = [[0; 30]; 12];
static mut TERM_HIST_LEN: [usize; 12] = [0; 12];

#[derive(Copy, Clone)]
struct Window { id: u8, x: usize, y: usize, w: usize, h: usize, title: &'static [u8], visible: bool, is_dragging: bool }

static mut WINDOWS: [Window; 4] = [
    Window { id: 1, x: 20, y: 15, w: 250, h: 150, title: b"TERMINAL", visible: false, is_dragging: false },
    Window { id: 2, x: 40, y: 30, w: 200, h: 100, title: b"FILE EXPLORER", visible: false, is_dragging: false },
    Window { id: 3, x: 80, y: 60, w: 180, h: 100, title: b"NOTES", visible: false, is_dragging: false },
    Window { id: 4, x: 100, y: 40, w: 160, h: 110, title: b"SETTINGS", visible: false, is_dragging: false },
];
static mut ACTIVE_WIN: usize = 0;
static mut DRAG_OFFSET_X: usize = 0;
static mut DRAG_OFFSET_Y: usize = 0;

pub fn start() {
    mouse::init();

    let mut last_click = false;

    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        let key = keyboard::read_key();
        unsafe { BLINK_FRAME = BLINK_FRAME.wrapping_add(1); }

        let clicked_now = is_clicked && !last_click;
        let key_pressed_now = key != 0; 

        unsafe {
            handle_desktop_clicks(mx, my, is_clicked, clicked_now);
            handle_keyboard(key, key_pressed_now);

            draw_desktop();
            draw_windows();

            if START_MENU_OPEN { draw_start_menu(); }
        }

        last_click = is_clicked;
        draw_cursor(mx, my);
        vga::swap_buffers();
    }
}

unsafe fn handle_desktop_clicks(mx: usize, my: usize, is_clicked: bool, clicked_now: bool) {
    let win_ptr = addr_of_mut!(WINDOWS) as *mut Window;
    
    for i in (0..4).rev() {
        let w = &mut *win_ptr.add(i);
        if w.is_dragging {
            if is_clicked {
                w.x = mx.saturating_sub(DRAG_OFFSET_X);
                w.y = my.saturating_sub(DRAG_OFFSET_Y);
            } else {
                w.is_dragging = false;
            }
            return;
        }
    }

    if !clicked_now { return; }

    if mx >= 2 && mx <= 47 && my >= 185 && my <= 198 { START_MENU_OPEN = !START_MENU_OPEN; return; }
    if START_MENU_OPEN {
        if mx >= 2 && mx <= 112 && my >= 70 && my <= 185 {
            if my > 75 && my < 90 { toggle_window(1); }
            else if my > 95 && my < 110 { toggle_window(2); }
            else if my > 115 && my < 130 { toggle_window(3); }
            else if my > 135 && my < 150 { toggle_window(4); }
        }
        START_MENU_OPEN = false; return;
    }

    if mx >= 10 && mx <= 40 && my >= 10 && my <= 40 { toggle_window(2); }
    if mx >= 10 && mx <= 40 && my >= 50 && my <= 80 { toggle_window(4); }

    for i in (0..4).rev() {
        let w = &mut *win_ptr.add(i);
        if w.visible && mx >= w.x && mx <= w.x + w.w && my >= w.y && my <= w.y + w.h {
            ACTIVE_WIN = i;
            if mx >= w.x + w.w - 14 && my <= w.y + 14 { w.visible = false; return; }

            if my >= w.y && my <= w.y + 12 {
                w.is_dragging = true;
                DRAG_OFFSET_X = mx.saturating_sub(w.x);
                DRAG_OFFSET_Y = my.saturating_sub(w.y);
            }

            if w.id == 4 {
                if mx > w.x + 10 && mx < w.x + 50 && my > w.y + 40 && my < w.y + 55 { *addr_of_mut!(BG_COLOR) = 3; }
                if mx > w.x + 60 && mx < w.x + 100 && my > w.y + 40 && my < w.y + 55 { *addr_of_mut!(BG_COLOR) = 1; }
            }
            return;
        }
    }
}

unsafe fn toggle_window(id: u8) {
    let win_ptr = addr_of_mut!(WINDOWS) as *mut Window;
    for i in 0..4 {
        if (*win_ptr.add(i)).id == id { (*win_ptr.add(i)).visible = true; ACTIVE_WIN = i; }
    }
}

unsafe fn handle_keyboard(key: u8, pressed: bool) {
    if !pressed || key == 0 { return; }

    let win_ptr = addr_of!(WINDOWS) as *const Window;
    let active_id = (*win_ptr.add(ACTIVE_WIN)).id;
    let is_visible = (*win_ptr.add(ACTIVE_WIN)).visible;

    if !is_visible { return; }

    if active_id == 1 {
        if key == 8 {
            if TERM_LEN > 0 { TERM_LEN -= 1; }
        } else if key == b'\n' {
            process_command();
        } else if key >= 32 && key <= 126 && TERM_LEN < 28 {
            let ptr = addr_of_mut!(TERM_BUF) as *mut u8;
            ptr.add(TERM_LEN).write(key);
            TERM_LEN += 1;
        }
    }
}

unsafe fn process_command() {
    let buf_slice = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
    push_history(buf_slice, TERM_LEN);
    TERM_LEN = 0;
}

unsafe fn push_history(text: &[u8], len: usize) {
    let hist_ptr = addr_of_mut!(TERM_HIST) as *mut [u8; 30];
    let lens_ptr = addr_of_mut!(TERM_HIST_LEN) as *mut usize;
    for i in 0..11 {
        let src = hist_ptr.add(i + 1) as *const u8;
        let dst = hist_ptr.add(i) as *mut u8;
        for j in 0..30 { *dst.add(j) = *src.add(j); }
        *lens_ptr.add(i) = *lens_ptr.add(i + 1);
    }
    let l = if len > 30 { 30 } else { len };
    let text_ptr = text.as_ptr();
    let dst = hist_ptr.add(11) as *mut u8;
    for i in 0..l { *dst.add(i) = *text_ptr.add(i); }
    *lens_ptr.add(11) = l;
}

fn get_time() -> (u8, u8) {
    unsafe {
        let read_cmos = |reg: u8| -> u8 {
            let val: u8; core::arch::asm!("out 0x70, al", "in al, 0x71", inout("al") reg => val); val
        };
        let m = read_cmos(0x02); let h = read_cmos(0x04);
        ( (h & 0x0F) + ((h / 16) * 10), (m & 0x0F) + ((m / 16) * 10) )
    }
}

fn draw_desktop() {
    vga::clear_screen(unsafe { BG_COLOR });
    draw_icon(10, 10, b"DISK");
    draw_icon(10, 50, b"SYS");

    draw_raised_rect(0, 185, 320, 15, 7);

    let is_open = unsafe { START_MENU_OPEN };
    if is_open { draw_sunken_rect(2, 187, 45, 11, 7); }
    else { draw_raised_rect(2, 187, 45, 11, 7); }

    vga::draw_str(b"START", 8, 189, 0);

    let (h, m) = get_time();
    let mut t_str = [b'0', b'0', b':', b'0', b'0'];
    t_str[0] = b'0' + (h / 10); t_str[1] = b'0' + (h % 10);
    t_str[3] = b'0' + (m / 10); t_str[4] = b'0' + (m % 10);
    
    draw_sunken_rect(275, 187, 42, 11, 7);
    vga::draw_str(&t_str, 282, 189, 0);
}

fn draw_start_menu() {
    draw_raised_rect(2, 70, 110, 115, 7);
    vga::draw_rect(4, 72, 20, 111, 1);

    vga::draw_str(b"O", 10, 130, 15); vga::draw_str(b"M", 10, 140, 15);
    vga::draw_str(b"N", 10, 150, 15); vga::draw_str(b"I", 10, 160, 15);
    vga::draw_str(b"X", 10, 170, 15);

    vga::draw_str(b"TERMINAL", 30, 78, 0); vga::draw_str(b"FILES", 30, 98, 0);
    vga::draw_str(b"NOTES", 30, 118, 0); vga::draw_str(b"SETTINGS", 30, 138, 0);

    vga::draw_rect(28, 153, 80, 1, 8); vga::draw_rect(28, 154, 80, 1, 15);
    vga::draw_str(b"SHUT DOWN", 30, 162, 0);
}

unsafe fn draw_windows() {
    let win_ptr = addr_of!(WINDOWS) as *const Window;
    for i in 0..4 {
        if i != ACTIVE_WIN && (*win_ptr.add(i)).visible { draw_app_window(&*win_ptr.add(i), false); }
    }
    if (*win_ptr.add(ACTIVE_WIN)).visible { draw_app_window(&*win_ptr.add(ACTIVE_WIN), true); }
}

unsafe fn draw_app_window(w: &Window, is_active: bool) {
    vga::draw_rect(w.x + 2, w.y + 2, w.w, w.h, 0); 
    draw_raised_rect(w.x, w.y, w.w, w.h, 7); 

    let title_c = if is_active { 1 } else { 8 };
    vga::draw_rect(w.x + 2, w.y + 2, w.w - 4, 12, title_c);
    vga::draw_str(w.title, w.x + 6, w.y + 4, 15);

    draw_raised_rect(w.x + w.w - 14, w.y + 2, 12, 12, 7);
    vga::draw_str(b"X", w.x + w.w - 11, w.y + 4, 0);

    if w.id == 1 {
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 0);
        let h_len_ptr = addr_of!(TERM_HIST_LEN) as *const usize;
        let h_ptr = addr_of!(TERM_HIST) as *const [u8; 30];

        for i in 0..12 {
            let len = *h_len_ptr.add(i);
            if len > 0 {
                let text = core::slice::from_raw_parts(h_ptr.add(i) as *const u8, len);
                vga::draw_str(text, w.x + 8, w.y + 20 + (i * 10), 10);
            }
        }

        let input_y = w.y + 130;
        vga::draw_str(b">", w.x + 8, input_y, 10);
        if TERM_LEN > 0 {
            let text = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
            vga::draw_str(text, w.x + 18, input_y, 10);
        }
        if is_active && (BLINK_FRAME % 60) < 30 { vga::draw_rect(w.x + 18 + (TERM_LEN * 8), input_y, 6, 8, 10); }
    }
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
