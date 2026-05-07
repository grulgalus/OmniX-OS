#![no_std]
#![no_main]

#[path = "../../drivers/rust/vga.rs"]
pub mod vga;
#[path = "../../drivers/rust/ata.rs"]
pub mod ata;

mod installer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("hlt"); } }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    installer::run_installer();
    loop { unsafe { core::arch::asm!("hlt"); } }
}
