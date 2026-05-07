pub struct User { pub uid: u8, pub admin: bool }
static mut CURRENT_USER: User = User { uid: 0, admin: true };
pub fn check_perm() -> bool { unsafe { CURRENT_USER.admin } }
