#[repr(C, packed)]
pub struct OmxApkHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub entry_point: u32,
    pub file_size: u32,
}

pub fn is_valid_app(header: &OmxApkHeader) -> bool {
    header.magic == [b'O', b'M', b'X', b'!']
}

pub fn run_application(name: &str) {
    crate::drivers::vga::print_str("Spoustim .omxapk: ");
    crate::drivers::vga::print_str(name);
    crate::drivers::vga::print_str("\n");
}
