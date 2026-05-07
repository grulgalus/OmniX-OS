pub fn list_files() {
    let mut sector = [0u8; 512];
    crate::ata::read_sector(1, &mut sector);
}
