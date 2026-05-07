use core::arch::asm;

unsafe fn outb(port: u16, val: u8) { asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags)); }
unsafe fn inb(port: u16) -> u8 { let mut val: u8; asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags)); val }
unsafe fn outw(port: u16, val: u16) { asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack, preserves_flags)); }
unsafe fn inw(port: u16) -> u16 { let mut val: u16; asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack, preserves_flags)); val }

pub fn write_sector(lba: u32, data: &[u8; 512]) {
    unsafe {
        let mut timeout = 0xFFFFFF;
        while (inb(0x1F7) & 0xC0) != 0x40 { timeout -= 1; if timeout == 0 { return; } }
        outb(0x1F2, 1); outb(0x1F3, lba as u8); outb(0x1F4, (lba >> 8) as u8); outb(0x1F5, (lba >> 16) as u8);
        outb(0x1F6, 0xE0 | ((lba >> 24) & 0x0F) as u8); outb(0x1F7, 0x30);
        while (inb(0x1F7) & 0x08) == 0 {}
        for i in 0..256 {
            let word = (data[i * 2] as u16) | ((data[i * 2 + 1] as u16) << 8);
            outw(0x1F0, word);
        }
    }
}

pub fn read_sector(lba: u32, data: &mut [u8; 512]) {
    unsafe {
        let mut timeout = 0xFFFFFF;
        while (inb(0x1F7) & 0xC0) != 0x40 { timeout -= 1; if timeout == 0 { return; } }
        outb(0x1F2, 1); outb(0x1F3, lba as u8); outb(0x1F4, (lba >> 8) as u8); outb(0x1F5, (lba >> 16) as u8);
        outb(0x1F6, 0xE0 | ((lba >> 24) & 0x0F) as u8); outb(0x1F7, 0x20); // 0x20 = Prikaz ke cteni
        while (inb(0x1F7) & 0x08) == 0 {}
        for i in 0..256 {
            let word = inw(0x1F0);
            data[i * 2] = (word & 0xFF) as u8;
            data[i * 2 + 1] = (word >> 8) as u8;
        }
    }
}
