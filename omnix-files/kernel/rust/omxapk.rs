#[derive(Copy, Clone)]
pub struct OmxApp<'a> {
    pub id: u8,
    pub name: &'a str,
    pub payload: &'a [u8],
}

pub const APP_COUNT: usize = 5;

// HLOUPÁ, ALE 100% FUNKČNÍ VERZE PRO TEST UI
pub fn get_default_apps() -> [OmxApp<'static>; APP_COUNT] {
    [
        OmxApp { id: 1, name: "Terminal", payload: crate::TERMINAL_APK },
        OmxApp { id: 2, name: "Explorer", payload: crate::EXPLORER_APK },
        OmxApp { id: 3, name: "Settings", payload: crate::SETTINGS_APK },
        OmxApp { id: 4, name: "Music",    payload: crate::SETTINGS_APK },
        OmxApp { id: 5, name: "Nano",     payload: crate::SETTINGS_APK },
    ]
}
