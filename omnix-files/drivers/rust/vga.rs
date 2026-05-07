const FRAMEBUFFER: *mut u8 = 0xA0000 as *mut u8;

pub fn put_pixel(x: usize, y: usize, color: u8) {
    if x < 320 && y < 200 {
        unsafe { *FRAMEBUFFER.add(y * 320 + x) = color; }
    }
}

pub fn clear_screen(color: u8) {
    unsafe {
        for i in 0..64000 {
            *FRAMEBUFFER.add(i) = color;
        }
    }
}

pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: u8) {
    for j in y..(y + h) {
        for i in x..(x + w) {
            put_pixel(i, j, color);
        }
    }
}
