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

    if app_data[0] != b'O' || app_data[1] != b'M' || app_data[2] != b'X' || app_data[3] != b'A' { return; }

    loop {
        vga::clear_screen(0); 

        let mut pc = 10; 
        while pc < 500 { // Kontrola limitu, aby kompilator neplakal
            unsafe {
                let opcode = *app_data.get_unchecked(pc);
                match opcode {
                    0x01 => { 
                        vga::draw_rect(*app_data.get_unchecked(pc+1) as usize, *app_data.get_unchecked(pc+2) as usize, *app_data.get_unchecked(pc+3) as usize, *app_data.get_unchecked(pc+4) as usize, *app_data.get_unchecked(pc+5));
                        pc += 6;
                    }
                    0x02 => { 
                        let len = *app_data.get_unchecked(pc+3) as usize;
                        if pc + 4 + len <= 512 {
                            vga::draw_str(&app_data[pc+4 .. pc+4+len], *app_data.get_unchecked(pc+1) as usize, *app_data.get_unchecked(pc+2) as usize, 15);
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
