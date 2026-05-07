use crate::vga;
use crate::ata;
use crate::system_ui;
use crate::keyboard;

pub fn run_installer() {
    vga::clear_screen(1); 
    
    // Vykresleni okna pro vyber partisny
    vga::draw_rect(30, 30, 260, 140, 7); 
    vga::draw_rect(30, 30, 260, 15, 1); 
    vga::draw_str(b"OMNIX OS SETUP - PARTITION SELECT", 34, 34, 15);

    vga::draw_str(b"SELECT TARGET DRIVE:", 40, 60, 0);
    
    // Disk 1
    vga::draw_rect(40, 80, 240, 20, 15);
    vga::draw_rect(40, 80, 240, 20, 8); 
    vga::draw_rect(41, 81, 238, 18, 15); 
    vga::draw_str(b"[1] /DEV/HDA - 64MB (PRIMARY)", 45, 86, 0);

    // Disk 2
    vga::draw_rect(40, 110, 240, 20, 15);
    vga::draw_rect(40, 110, 240, 20, 8);
    vga::draw_rect(41, 111, 238, 18, 15);
    vga::draw_str(b"[2] /DEV/HDB - UNALLOCATED", 45, 116, 0);

    // UKAŽ TO NA MONITORU!
    vga::swap_buffers();

    // Cekani na vyber disku
    loop {
        let key = keyboard::read_key();
        if key == b'1' || key == b'2' {
            break;
        }
    }

    // Prekresleni na formatovani
    vga::clear_screen(1); 
    vga::draw_rect(60, 50, 200, 100, 7); 
    vga::draw_rect(60, 50, 200, 15, 1); 
    vga::draw_str(b"INSTALLING OMNIX OS", 80, 54, 15);

    let mut disk_data = [0u8; 512];
    disk_data[0] = 0x4F; 
    disk_data[1] = 0x4D; 

    vga::draw_rect(140, 80, 50, 10, 7);

    for i in 0..101 {
        let width = i * 2;
        vga::draw_rect(60, 100, width as usize, 20, 10); // Zelena progress bar
        
        vga::draw_rect(140, 80, 40, 10, 7); 
        let mut text = [b'0', b'0', b'0', b'%'];
        text[0] = b'0' + (i / 100) as u8;
        text[1] = b'0' + ((i / 10) % 10) as u8;
        text[2] = b'0' + (i % 10) as u8;
        
        let start_idx = if i == 100 { 0 } else if i >= 10 { 1 } else { 2 };
        vga::draw_str(&text[start_idx..4], 140, 80, 0);

        // UKAŽ AKTUALNÍ PROCENTA NA MONITORU!
        vga::swap_buffers();

        ata::write_sector(i as u32, &disk_data);
        fake_delay(200000);
    }

    vga::draw_str(b"DONE!", 140, 130, 0);
    vga::swap_buffers(); // Ukaz napis DONE
    fake_delay(30000000);

    system_ui::start();
}

fn fake_delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)); }
    }
}
