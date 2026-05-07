use crate::vga;

pub struct OmxApkHeader {
    pub magic: u32,
    pub version: u8,
}

#[allow(dead_code)]
pub fn is_valid_app(header: &OmxApkHeader) -> bool {
    header.magic == 0x4F4D5800 
}

pub fn run_application(name: &str) {
    vga::clear_screen();
    vga::print_str("Spoustim OmniX aplikaci:\n");
    vga::print_str(name);
    vga::print_str("\n\nBezi... (Zavrete restartem GUI)");
}
