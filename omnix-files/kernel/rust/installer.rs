use crate::vga;
use crate::ata;
use crate::system_ui;

pub fn run_installer() {
    vga::clear_screen(1); 
    
    vga::draw_rect(60, 50, 200, 100, 7); 
    vga::draw_rect(60, 50, 200, 15, 8); 
    vga::draw_str(b"INSTALLING OMNIX OS", 80, 54, 15);

    let mut disk_data = [0u8; 512];
    disk_data[0] = 0x4F; 
    disk_data[1] = 0x4D; 

    vga::draw_rect(140, 80, 50, 10, 7);

    for i in 0..101 {
        let width = i * 2;
        vga::draw_rect(60, 100, width as usize, 20, 2); 
        
        vga::draw_rect(140, 80, 40, 10, 7); 
        let mut text = [b'0', b'0', b'0', b'%'];
        text[0] = b'0' + (i / 100) as u8;
        text[1] = b'0' + ((i / 10) % 10) as u8;
        text[2] = b'0' + (i % 10) as u8;
        
        let start_idx = if i == 100 { 0 } else if i >= 10 { 1 } else { 2 };
        vga::draw_str(&text[start_idx..4], 140, 80, 0);

        ata::write_sector(i as u32, &disk_data);
        fake_delay(300000);
    }

    vga::draw_str(b"DONE!", 140, 130, 0);
    fake_delay(30000000);

    system_ui::start();
}

fn fake_delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}
