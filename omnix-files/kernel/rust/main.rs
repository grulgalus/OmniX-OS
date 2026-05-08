#![no_std]
#![no_main]

#[path = "../../drivers/rust/vga.rs"]
pub mod vga;
#[path = "../../drivers/rust/ata.rs"]
pub mod ata;
#[path = "../../drivers/rust/mouse.rs"]
pub mod mouse;
#[path = "../../drivers/rust/keyboard.rs"]
pub mod keyboard;
#[path = "../../drivers/rust/rtc.rs"]
pub mod rtc;

pub mod installer;
pub mod system_ui;
pub mod omxapk;
pub mod pci;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { 
    loop { unsafe { core::arch::asm!("hlt"); } } 
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // installer::run_installer();
    system_ui::start();
    loop { unsafe { core::arch::asm!("hlt"); } }
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0; while i < n { *s.add(i) = c as u8; i += 1; } s
}
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0; while i < n { *dest.add(i) = *src.add(i); i += 1; } dest
}
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dest < src as *mut u8 { memcpy(dest, src, n) } else {
        let mut i = n; while i > 0 { i -= 1; *dest.add(i) = *src.add(i); } dest
    }
}
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0; while i < n {
        let a = *s1.add(i); let b = *s2.add(i);
        if a != b { return a as i32 - b as i32; } i += 1;
    } 0
}

crate::pci::find_intel_e1000()

if let Some(bar0) = pci::find_intel_e1000() {
    // BAR0 obsahuje adresu. Pokud má na konci 1, je to Port I/O, pokud 0, je to Memory Mapped.
    let adresa = bar0 & 0xFFFFFFF0; // Vyčistíme flagy
    
    // Tady předáme "adresu" tvému driveru (tohle přesně tvůj E1000 driver potřebuje!)
    net_driver::init(adresa); 
    
    // Pro debug (ať víš, že to jede) si to zatím můžeš vypsat (jestli máš nějaký print):
    println!("Intel E1000 nalezen na adrese: {:#X}", adresa);
} else {
    println!("Síťová karta nebyla nalezena.");
}
