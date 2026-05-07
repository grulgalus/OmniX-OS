static mut NOTIF: [u8; 32] = [0; 32];
pub fn push_notif(msg: &[u8]) { unsafe { NOTIF.copy_from_slice(msg); } }
