pub struct Window { pub id: u8, pub x: usize, pub y: usize, pub w: usize, pub h: usize, pub active: bool }
static mut WINDOWS: [Window; 8] = [Window { id: 0, x: 0, y: 0, w: 0, h: 0, active: false }; 8];
pub fn alt_tab() {
    unsafe {
        let mut active_idx = 0;
        for i in 0..8 { if WINDOWS[i].active { active_idx = i; WINDOWS[i].active = false; } }
        let next = (active_idx + 1) % 8;
        WINDOWS[next].active = true;
    }
}
