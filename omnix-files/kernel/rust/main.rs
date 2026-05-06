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

// Zde začíná náš nový, profi OS
#[no_mangle]
pub extern "C" fn kernel_main(_magic: u32, _info: u32) -> ! {
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
