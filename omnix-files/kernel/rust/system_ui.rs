use crate::vga;
use crate::mouse;
use crate::keyboard;
use core::ptr::addr_of_mut;

// Rozšířené stavy OS
static mut OS_STATE: u8 = 0; // 0 = Boot animace, 1 = Windows Login, 2 = Desktop
static mut IS_ADMIN: bool = false;
static mut BG_COLOR: u8 = 3; 
static mut BOOT_PROGRESS: u32 = 0; // Pro animaci

static mut NOTIF_MSG: &'static [u8] = b"";
static mut NOTIF_TIMER: u16 = 0;

static mut START_MENU_OPEN: bool = false;
static mut BLINK_FRAME: u32 = 0;         

// Větší display = větší plocha
const SCREEN_W: usize = 640;
const SCREEN_H: usize = 480;

pub fn start() {
    mouse::init();

    let mut last_click = false;
    loop {
        let (mx, my, is_clicked) = mouse::get_state();
        unsafe { BLINK_FRAME = BLINK_FRAME.wrapping_add(1); }
        let clicked_now = is_clicked && !last_click;
        
        unsafe {
            if OS_STATE == 0 {
                // --- 1. BOOT ANIMACE ---
                draw_boot_animation();
                BOOT_PROGRESS += 1;
                if BOOT_PROGRESS > 150 { OS_STATE = 1; } // Po chvíli jde na Login
            } 
            else if OS_STATE == 1 {
                // --- 2. WINDOWS LOGIN ---
                draw_windows_login(mx, my);
                if clicked_now {
                    // Kliknutí na Admina
                    if mx > 270 && mx < 370 && my > 250 && my < 280 {
                        IS_ADMIN = true; OS_STATE = 2; show_notif(b"WELCOME!");
                    }
                    // Kliknutí na Usera
                    if mx > 270 && mx < 370 && my > 290 && my < 320 {
                        IS_ADMIN = false; OS_STATE = 2; show_notif(b"WELCOME!");
                    }
                }
            } 
            else {
                // --- 3. DESKTOP ---
                draw_desktop();
                // Tady by se vykreslovala okna jako předtím
            }
        }
        
        last_click = is_clicked;
        if unsafe { OS_STATE } > 0 { draw_cursor(mx, my); } // Kurzor jen po bootu
        vga::swap_buffers();
    }
}

// ------------------------------
// --- BOOT ANIMACE (STARTUP) ---
// ------------------------------
unsafe fn draw_boot_animation() {
    vga::clear_screen(0); // Černé pozadí jako při startu PC
    
    // Logo OmniX (vycentrované pro 320x200)
    vga::draw_str(b"OMNIX OS", 120, 80, 15);
    vga::draw_str(b"Starting...", 110, 100, 7);

    // Načítací proužek
    draw_sunken_rect(60, 120, 200, 15, 0); // Prázdný rámeček (zmenšený)
    let bar_width = if BOOT_PROGRESS > 196 { 196 } else { BOOT_PROGRESS as usize };
    vga::draw_rect(62, 122, bar_width, 11, 2); // Zelená výplň
}

// ------------------------------
// --- WINDOWS LOGIN SCREEN -----
// ------------------------------
fn draw_windows_login(mx: usize, my: usize) {
    vga::clear_screen(1); // Modré Windows pozadí

    // Avatar uživatele (posunutý doleva a zmenšený)
    draw_raised_rect(130, 40, 60, 60, 7);
    vga::draw_rect(145, 50, 30, 30, 15); // Hlava
    vga::draw_rect(135, 85, 50, 15, 15); // Tělo

    vga::draw_str(b"OMNIX", 140, 110, 15);

    // Tlačítko ADMIN
    let admin_c = if mx > 110 && mx < 210 && my > 130 && my < 150 { 10 } else { 7 };
    draw_raised_rect(110, 130, 100, 20, admin_c);
    vga::draw_str(b"ADMIN", 140, 136, 0);

    // Tlačítko USER
    let user_c = if mx > 110 && mx < 210 && my > 160 && my < 180 { 10 } else { 7 };
    draw_raised_rect(110, 160, 100, 20, user_c);
    vga::draw_str(b"USER", 145, 166, 0);
}

fn draw_desktop() {
    vga::clear_screen(unsafe { BG_COLOR });
    draw_raised_rect(0, 180, 320, 20, 7); // Spodní lišta (320px)
    draw_raised_rect(2, 182, 45, 16, 10);
    vga::draw_str(b"START", 8, 186, 0);
}

fn show_notif(msg: &'static [u8]) {
    unsafe { *addr_of_mut!(NOTIF_MSG) = msg; *addr_of_mut!(NOTIF_TIMER) = 200; }
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
