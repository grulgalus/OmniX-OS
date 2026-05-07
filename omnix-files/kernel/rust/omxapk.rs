use crate::vga;
use crate::ata;
use crate::keyboard;

pub fn install_demo_app(lba: u32) {
    let mut apk = [0u8; 512];
    
    apk[0] = b'O'; apk[1] = b'M'; apk[2] = b'X'; apk[3] = b'A';
    
    apk[10] = 0x01; apk[11] = 60; apk[12] = 40; apk[13] = 200; apk[14] = 120; apk[15] = 9; 
    apk[16] = 0x01; apk[17] = 60; apk[18] = 40; apk[19] = 200; apk[20] = 15; apk[21] = 1;  

    apk[22] = 0x02; apk[23] = 65; apk[24] = 44; apk[25] = 17; 
    let t1 = b"OMNIX DEMO APP 3D"; for i in 0..17 { apk[26+i] = t1[i]; }

    apk[43] = 0x02; apk[44] = 100; apk[45] = 80; apk[46] = 15; 
    let t2 = b"HELLO FROM DISK"; for i in 0..15 { apk[47+i] = t2[i]; }

    apk[62] = 0xFF;

    ata::write_sector(lba, &apk);
}

pub fn run_app(lba: u32) {
    let mut app_data = [0u8; 512];
    ata::read_sector(lba, &mut app_data);

    if app_data[0] != b'O' || app_data[1] != b'M' || app_data[2] != b'X' || app_data[3] != b'A' { 
        return; 
    }

    loop {
        vga::clear_screen(0); 

        let mut pc = 10; 
        let data_ptr = app_data.as_ptr(); // <- TADY bereme raw pointer na zacatek pole

        while pc < 500 { 
            unsafe {
                let opcode = *data_ptr.add(pc); // Nacteme instrukci pres pointer

                match opcode {
                    0x01 => { 
                        // Rozepsano pro prehlednost misto dlouheho radku
                        let x = *data_ptr.add(pc + 1) as usize;
                        let y = *data_ptr.add(pc + 2) as usize;
                        let w = *data_ptr.add(pc + 3) as usize;
                        let h = *data_ptr.add(pc + 4) as usize;
                        let color = *data_ptr.add(pc + 5);
                        
                        vga::draw_rect(x, y, w, h, color);
                        pc += 6;
                    }
                    0x02 => { 
                        let x = *data_ptr.add(pc + 1) as usize;
                        let y = *data_ptr.add(pc + 2) as usize;
                        let len = *data_ptr.add(pc + 3) as usize;
                        
                        if pc + 4 + len <= 512 {
                            // Vezmeme adresu zacatku textu a udelame z ni bezpecny string/slice
                            let text_ptr = data_ptr.add(pc + 4);
                            let text = core::slice::from_raw_parts(text_ptr, len);
                            
                            vga::draw_str(text, x, y, 15);
                        }
                        pc += 4 + len;
                    }
                    0xFF => break, 
                    _ => break, 
                }
            }
        }

        vga::draw_str(b"[PRESS ESC TO CLOSE APP]", 70, 180, 12);
        vga::swap_buffers();

        if keyboard::read_key() == 27 { break; }
    }
}
