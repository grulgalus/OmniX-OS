use core::arch::asm;

// Přímý zápis na porty základní desky (0xCF8 a 0xCFC)
unsafe fn out32(port: u16, val: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") val);
}

unsafe fn in32(port: u16) -> u32 {
    let val: u32;
    asm!("in eax, dx", out("eax") val, in("dx") port);
    val
}

// Čtení 16-bitové hodnoty z PCI
pub fn pci_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
                  ((func as u32) << 8) | (offset as u32 & 0xFC) | 0x80000000;
    unsafe {
        out32(0xCF8, address);
        ((in32(0xCFC) >> ((offset & 2) * 8)) & 0xFFFF) as u16
    }
}

// Čtení 32-bitové hodnoty (BAR paměťových adres)
pub fn pci_read_dword(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
                  ((func as u32) << 8) | (offset as u32 & 0xFC) | 0x80000000;
    unsafe {
        out32(0xCF8, address);
        in32(0xCFC)
    }
}

// Funkce, která projde celou základní desku a najde Intel E1000
pub fn find_intel_e1000() -> Option<u32> {
    for bus in 0..=255 {
        for slot in 0..32 {
            let vendor = pci_read_word(bus, slot, 0, 0);
            if vendor != 0xFFFF {
                let device = pci_read_word(bus, slot, 0, 2);
                
                // 0x8086 je oficiální kód pro INTEL!
                // Pokud narazíme na Intel síťovku, vytáhneme její BAR0 (Base Address)
                if vendor == 0x8086 {
                    let bar0 = pci_read_dword(bus, slot, 0, 0x10);
                    return Some(bar0);
                }
            }
        }
    }
    None
}
