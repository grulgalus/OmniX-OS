pub fn set_brightness(val: u8) { crate::drivers::vga::set_reg(0x3C8, val); }
pub fn set_volume(val: u8) { crate::drivers::audio::out(0x220, val); }
