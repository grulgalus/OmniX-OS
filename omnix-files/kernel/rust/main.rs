#![no_std]
#![no_main]

#[path = "../../drivers/rust/vga.rs"]
pub mod vga;

#[path = "../../drivers/rust/keyboard.rs"]
pub mod keyboard;

mod installer;
mod omxapk;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Zde zacina nas novy, profi OS
#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::set_color(0x0F, 0x05);
    vga::clear_screen();

    let is_installed = false;

    if !is_installed {
        installer::run_installer();
    } else {
        omxapk::run_application("system_ui.omxapk");
    }

    loop {
        let key = keyboard::read_key();
        if key != 0 {
            vga::print_char(key);
        }
    }
}
