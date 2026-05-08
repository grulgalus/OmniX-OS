use crate::vga;
use crate::mouse;
use crate::keyboard;
use crate::omxapk;
use core::ptr::{addr_of, addr_of_mut};

static mut BG_COLOR: u8 = 3;
static mut START_MENU_OPEN: bool = false;
static mut BLINK_FRAME: u32 = 0;

static mut TERM_BUF: [u8; 40] = [0; 40];
static mut TERM_LEN: usize = 0;
static mut TERM_HIST: [[u8; 40]; 10] = [[0; 40]; 10];
static mut TERM_HIST_LEN: [usize; 10] = [0; 10];

static mut NANO_BUF: [u8; 300] = [0; 300];
static mut NANO_LEN: usize = 0;

#[derive(Copy, Clone)]
struct Window {
    app_id: u8,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    visible: bool,
    is_dragging: bool,
    maximized: bool,
    old_x: usize,
    old_y: usize,
    old_w: usize,
    old_h: usize,
}

static mut WINDOWS: [Window; omxapk::APP_COUNT] = [
    Window { app_id: 1, x: 10, y: 10, w: 260, h: 150, visible: false, is_dragging: false, maximized: false, old_x: 0, old_y: 0, old_w: 0, old_h: 0 },
    Window { app_id: 2, x: 30, y: 30, w: 220, h: 120, visible: false, is_dragging: false, maximized: false, old_x: 0, old_y: 0, old_w: 0, old_h: 0 },
    Window { app_id: 3, x: 50, y: 50, w: 240, h: 130, visible: false, is_dragging: false, maximized: false, old_x: 0, old_y: 0, old_w: 0, old_h: 0 },
    Window { app_id: 4, x: 70, y: 40, w: 180, h: 110, visible: false, is_dragging: false, maximized: false, old_x: 0, old_y: 0, old_w: 0, old_h: 0 },
    Window { app_id: 5, x: 90, y: 60, w: 160, h: 90,  visible: false, is_dragging: false, maximized: false, old_x: 0, old_y: 0, old_w: 0, old_h: 0 },
];

static mut ACTIVE_WIN: usize = 0;
static mut DRAG_OFFSET_X: usize = 0;
static mut DRAG_OFFSET_Y: usize = 0;

fn get_apps() -> [omxapk::OmxApp; omxapk::APP_COUNT] {
    omxapk::get_default_apps()
}

fn get_app_by_id(id: u8) -> omxapk::OmxApp {
    let apps = get_apps();
    let mut i = 0;
    while i < omxapk::APP_COUNT {
        if apps[i].id == id {
            return apps[i];
        }
        i += 1;
    }
    apps[0]
}

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

unsafe fn play_sound(freq: u32) {
    if freq == 0 { return; }
    let div = 1193180 / freq;
    core::arch::asm!("out 0x43, al", in("al") 0xb6_u8);
    core::arch::asm!("out 0x42, al", in("al") (div & 0xFF) as u8);
    core::arch::asm!("out 0x42, al", in("al") (div >> 8) as u8);
    let mut tmp: u8;
    core::arch::asm!("in al, 0x61", out("al") tmp);
    if tmp & 3 != 3 { core::arch::asm!("out 0x61, al", in("al") tmp | 3); }
}

unsafe fn stop_sound() {
    let mut tmp: u8;
    core::arch::asm!("in al, 0x61", out("al") tmp);
    core::arch::asm!("out 0x61, al", in("al") tmp & 0xFC);
}

unsafe fn handle_desktop_clicks(mx: usize, my: usize, is_clicked: bool, clicked_now: bool) {
    let win_ptr = addr_of_mut!(WINDOWS) as *mut Window;

    for i in (0..omxapk::APP_COUNT).rev() {
        let w = &mut *win_ptr.add(i);
        if w.is_dragging && !w.maximized {
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

    if mx >= 2 && mx <= 57 && my >= 185 && my <= 198 {
        START_MENU_OPEN = !START_MENU_OPEN;
        return;
    }

    if START_MENU_OPEN {
        let apps = get_apps();
        let mut i = 0;
        while i < omxapk::APP_COUNT {
            let item_y = 58 + (i * 20);
            if mx >= 30 && mx <= 110 && my >= item_y && my <= item_y + 12 {
                toggle_window(apps[i].id);
                START_MENU_OPEN = false;
                return;
            }
            i += 1;
        }
        START_MENU_OPEN = false;
        return;
    }

    let apps = get_apps();
    let mut i = 0;
    while i < 2 {
        let icon_y = 10 + (i * 40);
        if mx >= 10 && mx <= 40 && my >= icon_y && my <= icon_y + 30 {
            toggle_window(apps[i + 1].id);
            return;
        }
        i += 1;
    }

    for i in (0..omxapk::APP_COUNT).rev() {
        let w = &mut *win_ptr.add(i);
        if w.visible && mx >= w.x && mx <= w.x + w.w && my >= w.y && my <= w.y + w.h {
            ACTIVE_WIN = i;

            if mx >= w.x + w.w - 14 && my >= w.y + 2 && my <= w.y + 14 {
                w.visible = false;
                stop_sound();
                return;
            }

            if mx >= w.x + w.w - 28 && mx <= w.x + w.w - 16 && my >= w.y + 2 && my <= w.y + 14 {
                if w.maximized {
                    w.x = w.old_x;
                    w.y = w.old_y;
                    w.w = w.old_w;
                    w.h = w.old_h;
                    w.maximized = false;
                } else {
                    w.old_x = w.x;
                    w.old_y = w.y;
                    w.old_w = w.w;
                    w.old_h = w.h;
                    w.x = 0;
                    w.y = 0;
                    w.w = 320;
                    w.h = 184;
                    w.maximized = true;
                }
                return;
            }

            if my >= w.y && my <= w.y + 14 && !w.maximized {
                w.is_dragging = true;
                DRAG_OFFSET_X = mx.saturating_sub(w.x);
                DRAG_OFFSET_Y = my.saturating_sub(w.y);
            }

            if w.app_id == 4 {
                if mx > w.x + 10 && mx < w.x + 70 && my > w.y + 40 && my < w.y + 55 { BG_COLOR = 3; }
                if mx > w.x + 80 && mx < w.x + 140 && my > w.y + 40 && my < w.y + 55 { BG_COLOR = 1; }
            }

            if w.app_id == 5 {
                if mx > w.x + 10 && mx < w.x + 50 && my > w.y + 30 && my < w.y + 45 { play_sound(440); }
                if mx > w.x + 60 && mx < w.x + 100 && my > w.y + 30 && my < w.y + 45 { play_sound(523); }
                if mx > w.x + 10 && mx < w.x + 50 && my > w.y + 60 && my < w.y + 75 { stop_sound(); }
            }

            return;
        }
    }
}

unsafe fn toggle_window(app_id: u8) {
    let win_ptr = addr_of_mut!(WINDOWS) as *mut Window;
    let mut i = 0;
    while i < omxapk::APP_COUNT {
        if (*win_ptr.add(i)).app_id == app_id {
            (*win_ptr.add(i)).visible = true;
            ACTIVE_WIN = i;
            return;
        }
        i += 1;
    }
}

unsafe fn handle_keyboard(key: u8, pressed: bool) {
    if !pressed || key == 0 { return; }

    let win_ptr = addr_of!(WINDOWS) as *const Window;
    let active = &*win_ptr.add(ACTIVE_WIN);

    if !active.visible { return; }

    if active.app_id == 1 {
        if key == 8 {
            if TERM_LEN > 0 { TERM_LEN -= 1; }
        } else if key == b'\n' {
            process_command();
        } else if key >= 32 && key <= 126 && TERM_LEN < 38 {
            let ptr = addr_of_mut!(TERM_BUF) as *mut u8;
            ptr.add(TERM_LEN).write(key);
            TERM_LEN += 1;
        }
    } else if active.app_id == 3 {
        if key == 8 {
            if NANO_LEN > 0 { NANO_LEN -= 1; }
        } else if key == b'\n' {
            if NANO_LEN < 290 {
                let ptr = addr_of_mut!(NANO_BUF) as *mut u8;
                ptr.add(NANO_LEN).write(b' ');
                NANO_LEN += 1;
            }
        } else if key >= 32 && key <= 126 && NANO_LEN < 290 {
            let ptr = addr_of_mut!(NANO_BUF) as *mut u8;
            ptr.add(NANO_LEN).write(key);
            NANO_LEN += 1;
        }
    }
}

unsafe fn process_command() {
    let buf_slice = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
    push_history(buf_slice, TERM_LEN);
    TERM_LEN = 0;
}

unsafe fn push_history(text: &[u8], len: usize) {
    let hist_ptr = addr_of_mut!(TERM_HIST) as *mut [u8; 40];
    let lens_ptr = addr_of_mut!(TERM_HIST_LEN) as *mut usize;
    let mut i = 0;
    while i < 9 {
        let src = hist_ptr.add(i + 1) as *const u8;
        let dst = hist_ptr.add(i) as *mut u8;
        let mut j = 0;
        while j < 40 {
            *dst.add(j) = *src.add(j);
            j += 1;
        }
        *lens_ptr.add(i) = *lens_ptr.add(i + 1);
        i += 1;
    }
    let l = if len > 40 { 40 } else { len };
    let text_ptr = text.as_ptr();
    let dst = hist_ptr.add(9) as *mut u8;
    let mut k = 0;
    while k < l {
        *dst.add(k) = *text_ptr.add(k);
        k += 1;
    }
    *lens_ptr.add(9) = l;
}

fn get_time() -> (u8, u8) {
    unsafe {
        let read_cmos = |reg: u8| -> u8 {
            let val: u8;
            core::arch::asm!("out 0x70, al", "in al, 0x71", inout("al") reg => val);
            val
        };
        let m = read_cmos(0x02);
        let h = read_cmos(0x04);
        ((h & 0x0F) + ((h / 16) * 10), (m & 0x0F) + ((m / 16) * 10))
    }
}

fn draw_desktop() {
    vga::clear_screen(unsafe { BG_COLOR });

    let apps = get_apps();
    draw_icon(10, 10, apps[1].icon_label);
    draw_icon(10, 50, apps[2].icon_label);

    draw_raised_rect(0, 185, 320, 15, 7);

    let is_open = unsafe { START_MENU_OPEN };
    if is_open { draw_sunken_rect(2, 187, 55, 11, 7); }
    else { draw_raised_rect(2, 187, 55, 11, 7); }

    vga::draw_str(b"START", 8, 189, 0);

    let (h, m) = get_time();
    let mut t_str = [b'0', b'0', b':', b'0', b'0'];
    t_str[0] = b'0' + (h / 10);
    t_str[1] = b'0' + (h % 10);
    t_str[3] = b'0' + (m / 10);
    t_str[4] = b'0' + (m % 10);

    draw_sunken_rect(260, 187, 55, 11, 7);
    vga::draw_str(&t_str, 265, 189, 0);
}

fn draw_start_menu() {
    draw_raised_rect(2, 50, 110, 135, 7);
    vga::draw_rect(4, 52, 20, 131, 1);
    vga::draw_str(b"O", 10, 110, 15);
    vga::draw_str(b"M", 10, 120, 15);
    vga::draw_str(b"N", 10, 130, 15);
    vga::draw_str(b"I", 10, 140, 15);
    vga::draw_str(b"X", 10, 150, 15);

    let apps = get_apps();
    let mut i = 0;
    while i < omxapk::APP_COUNT {
        vga::draw_str(apps[i].title, 30, 58 + (i * 20), 0);
        i += 1;
    }

    vga::draw_rect(28, 153, 80, 1, 8);
    vga::draw_rect(28, 154, 80, 1, 15);
    vga::draw_str(b"BOOT APPS", 30, 162, 0);
}

unsafe fn draw_windows() {
    let win_ptr = addr_of!(WINDOWS) as *const Window;
    let mut i = 0;
    while i < omxapk::APP_COUNT {
        if i != ACTIVE_WIN && (*win_ptr.add(i)).visible {
            draw_app_window(&*win_ptr.add(i), false);
        }
        i += 1;
    }
    if (*win_ptr.add(ACTIVE_WIN)).visible {
        draw_app_window(&*win_ptr.add(ACTIVE_WIN), true);
    }
}

unsafe fn draw_app_window(w: &Window, is_active: bool) {
    let app = get_app_by_id(w.app_id);

    if !w.maximized {
        vga::draw_rect(w.x + 2, w.y + 2, w.w, w.h, 0);
    }

    draw_raised_rect(w.x, w.y, w.w, w.h, 7);

    let title_c = if is_active { 1 } else { 8 };
    vga::draw_rect(w.x + 2, w.y + 2, w.w - 4, 12, title_c);
    vga::draw_str(app.title, w.x + 6, w.y + 4, 15);

    draw_raised_rect(w.x + w.w - 14, w.y + 2, 12, 12, 7);
    vga::draw_str(b"X", w.x + w.w - 11, w.y + 4, 0);

    draw_raised_rect(w.x + w.w - 28, w.y + 2, 12, 12, 7);
    vga::draw_str(b"[]", w.x + w.w - 27, w.y + 4, 0);

    if w.app_id == 1 {
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 0);
        let h_len_ptr = addr_of!(TERM_HIST_LEN) as *const usize;
        let h_ptr = addr_of!(TERM_HIST) as *const [u8; 40];
        let max_lines = if w.maximized { 15 } else { 8 };

        let mut i = 0;
        while i < max_lines {
            if i < 10 {
                let len = *h_len_ptr.add(i);
                if len > 0 {
                    let text = core::slice::from_raw_parts(h_ptr.add(i) as *const u8, len);
                    vga::draw_str(text, w.x + 8, w.y + 20 + (i * 10), 10);
                }
            }
            i += 1;
        }

        let input_y = w.y + 20 + (max_lines * 10);
        vga::draw_str(b">", w.x + 8, input_y, 10);
        if TERM_LEN > 0 {
            let text = core::slice::from_raw_parts(addr_of!(TERM_BUF) as *const u8, TERM_LEN);
            vga::draw_str(text, w.x + 18, input_y, 10);
        }
        if is_active && (BLINK_FRAME % 60) < 30 {
            vga::draw_rect(w.x + 18 + (TERM_LEN * 8), input_y, 6, 8, 10);
        }
    } else if w.app_id == 2 {
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 15);
        vga::draw_str(b"[BOOT]", w.x + 10, w.y + 25, 0);
        vga::draw_str(b"  -> default-apps", w.x + 10, w.y + 40, 0);
        vga::draw_str(b"  -> terminal.omxapk", w.x + 10, w.y + 55, 0);
        vga::draw_str(b"  -> nano.omxapk", w.x + 10, w.y + 70, 0);
    } else if w.app_id == 3 {
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 15);
        let chars_per_line = (w.w - 16) / 8;
        let mut start = 0;
        let mut cy = w.y + 20;
        while start < NANO_LEN {
            let end = if start + chars_per_line > NANO_LEN { NANO_LEN } else { start + chars_per_line };
            let text = core::slice::from_raw_parts(addr_of!(NANO_BUF[start]), end - start);
            vga::draw_str(text, w.x + 8, cy, 0);
            start += chars_per_line;
            cy += 12;
        }
        if is_active && chars_per_line > 0 && (BLINK_FRAME % 60) < 30 {
            let row = NANO_LEN / chars_per_line;
            let col = NANO_LEN % chars_per_line;
            vga::draw_rect(w.x + 8 + (col * 8), w.y + 20 + (row * 12), 6, 8, 0);
        }
    } else if w.app_id == 4 {
        vga::draw_str(b"BACKGROUND:", w.x + 10, w.y + 25, 0);
        draw_raised_rect(w.x + 10, w.y + 40, 60, 15, 7);
        vga::draw_str(b"CYAN", w.x + 18, w.y + 44, 0);
        draw_raised_rect(w.x + 80, w.y + 40, 60, 15, 7);
        vga::draw_str(b"BLUE", w.x + 90, w.y + 44, 0);
    } else if w.app_id == 5 {
        draw_sunken_rect(w.x + 4, w.y + 16, w.w - 8, w.h - 20, 0);
        draw_raised_rect(w.x + 10, w.y + 30, 40, 15, 7);
        vga::draw_str(b"BEEP", w.x + 14, w.y + 34, 0);
        draw_raised_rect(w.x + 60, w.y + 30, 40, 15, 7);
        vga::draw_str(b"BOOP", w.x + 64, w.y + 34, 0);
        draw_raised_rect(w.x + 10, w.y + 60, 40, 15, 7);
        vga::draw_str(b"STOP", w.x + 14, w.y + 64, 0);
    }
}

fn draw_icon(x: usize, y: usize, label: &[u8]) {
    draw_raised_rect(x + 4, y, 20, 18, 7);
    vga::draw_rect(x + 8, y + 4, 12, 10, 1);
    vga::draw_rect(x, y + 20, 30, 9, 1);
    vga::draw_str(label, x + 2, y + 22, 15);
}

fn draw_cursor(x: usize, y: usize) {
    vga::draw_rect(x, y, 2, 6, 15);
    vga::draw_rect(x, y, 5, 2, 15);
    vga::draw_rect(x + 2, y + 2, 2, 2, 15);
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

// app je struktura OmxApp, kterou jsi dostal z parseru
unsafe {
    crate::executor::run_omx_app(&app);
}
