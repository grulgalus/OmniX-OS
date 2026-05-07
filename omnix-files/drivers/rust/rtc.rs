use core::arch::asm;
use core::ptr::{addr_of, addr_of_mut};

static mut TIME_STR: [u8; 5] = [b'0', b'0', b':', b'0', b'0'];

pub unsafe fn get_time() -> &'static [u8] {
    let mut h: u8; let mut m: u8;
    asm!("out dx, al", in("dx") 0x70u16, in("al") 0x04u8, options(nomem, nostack, preserves_flags));
    asm!("in al, dx", out("al") h, in("dx") 0x71u16, options(nomem, nostack, preserves_flags));
    asm!("out dx, al", in("dx") 0x70u16, in("al") 0x02u8, options(nomem, nostack, preserves_flags));
    asm!("in al, dx", out("al") m, in("dx") 0x71u16, options(nomem, nostack, preserves_flags));
    
    let h_dec = (h & 0x0F) + ((h / 16) * 10);
    let m_dec = (m & 0x0F) + ((m / 16) * 10);
    
    let ptr = addr_of_mut!(TIME_STR);
    (*ptr)[0] = b'0' + (h_dec / 10); 
    (*ptr)[1] = b'0' + (h_dec % 10);
    (*ptr)[3] = b'0' + (m_dec / 10); 
    (*ptr)[4] = b'0' + (m_dec % 10);
    
    // Bezpecny navrat static reference
    &*addr_of!(TIME_STR)
}
