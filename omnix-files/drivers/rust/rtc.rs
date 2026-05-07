use core::arch::asm;
static mut TIME_STR: [u8; 5] = [b'0', b'0', b':', b'0', b'0'];

pub unsafe fn get_time() -> &'static [u8] {
    let mut h: u8; 
    let mut m: u8;
    asm!("out dx, al", in("dx") 0x70u16, in("al") 0x04u8, options(nomem, nostack, preserves_flags));
    asm!("in al, dx", out("al") h, in("dx") 0x71u16, options(nomem, nostack, preserves_flags));
    asm!("out dx, al", in("dx") 0x70u16, in("al") 0x02u8, options(nomem, nostack, preserves_flags));
    asm!("in al, dx", out("al") m, in("dx") 0x71u16, options(nomem, nostack, preserves_flags));
    
    let h_dec = (h & 0x0F) + ((h / 16) * 10);
    let m_dec = (m & 0x0F) + ((m / 16) * 10);
    
    TIME_STR[0] = b'0' + (h_dec / 10); 
    TIME_STR[1] = b'0' + (h_dec % 10);
    TIME_STR[3] = b'0' + (m_dec / 10); 
    TIME_STR[4] = b'0' + (m_dec % 10);
    
    &TIME_STR
}
