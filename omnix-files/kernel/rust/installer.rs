use crate::vga;
use crate::ata;

pub fn run_installer() {
    vga::clear_screen(1); 
    
    vga::draw_rect(60, 50, 200, 100, 7); 
    vga::draw_rect(60, 50, 200, 15, 8); 

    fake_delay(20000000);

    let mut disk_data = [0u8; 512];
    disk_data[0] = 0x4F; 
    disk_data[1] = 0x4D; 
    disk_data[2] = 0x4E; 
    disk_data[3] = 0x49; 
    disk_data[4] = 0x58; 

    for i in 0..160 {
        vga::draw_rect(80, 100, i, 20, 2); 
        ata::write_sector(i as u32, &disk_data);
        fake_delay(500000);
    }

    vga::draw_rect(60, 50, 200, 100, 2); 
    
    loop { unsafe { core::arch::asm!("hlt"); } }
}

fn fake_delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}
