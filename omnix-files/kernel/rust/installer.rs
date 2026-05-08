// installer.rs
use core::arch::asm;

// Pomocné funkce pro čtení a zápis na hardwarové porty
unsafe fn outb(port: u16, value: u8) { asm!("out dx, al", in("dx") port, in("al") value); }
unsafe fn inb(port: u16) -> u8 { let value: u8; asm!("in al, dx", out("al") value, in("dx") port); value }
unsafe fn outw(port: u16, value: u16) { asm!("out dx, ax", in("dx") port, in("ax") value); }

/// SKUTEČNÝ zápis na pevný disk (ATA PIO LBA28 Mode)
pub unsafe fn write_sector(lba: u32, data: &[u8; 512]) {
    // 1. Výběr disku (Master) a LBA adresy
    outb(0x1F2, 1); // Počet sektorů = 1
    outb(0x1F3, lba as u8);
    outb(0x1F4, (lba >> 8) as u8);
    outb(0x1F5, (lba >> 16) as u8);
    outb(0x1F6, 0xE0 | ((lba >> 24) & 0x0F) as u8);
    
    // 2. Příkaz k zápisu (Write Sectors = 0x30)
    outb(0x1F7, 0x30);
    
    // 3. Čekání, až bude disk připraven (Polling)
    while (inb(0x1F7) & 0x08) == 0 {}
    
    // 4. Zápis samotných dat (po 2 bajtech - 16 bitů)
    for i in 0..256 {
        let word = (data[i * 2] as u16) | ((data[i * 2 + 1] as u16) << 8);
        outw(0x1F0, word);
    }
    
    // 5. Cache flush k potvrzení zápisu
    outb(0x1F7, 0xE7);
}

pub fn run_real_installation() {
    // Tady bys reálně vzal bajty OS v paměti a začal je sektor po sektoru
    // zapisovat na disk. Toto přepíše data na virtuálním pevném disku!
    let dummy_sector = [0; 512]; // Sem by přišly reálné data
    unsafe { write_sector(100, &dummy_sector); }
}
