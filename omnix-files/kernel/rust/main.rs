#![no_std]
#![no_main]

#[path = "../../drivers/rust/vga.rs"]
pub mod vga;
#[path = "../../drivers/rust/keyboard.rs"]
pub mod keyboard;

mod installer;
mod omxapk;
mod system_ui;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::set_color(0x0F, 0x05);
    vga::clear_screen();

    installer::run_installer();
    system_ui::start();

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
