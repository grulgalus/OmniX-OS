use core::str;

// 1. Přejmenováno z OmxApk na OmxApp, přesně jak chce executor.rs!
#[derive(Clone, Copy)]
pub struct OmxApp<'a> {
    pub num_id: u8,
    pub name: &'a str,
    pub pkg_id: &'a str,
    pub payload: &'a [u8],
}

// 2. Přidáno APP_COUNT, které system_ui.rs používá pro for cykly
pub const APP_COUNT: usize = 3;

// Zůstávají natvrdo definované aplikace (s prázdným payloadem)
pub const DEFAULT_APPS: [OmxApp<'static>; APP_COUNT] = [
    OmxApp {
        num_id: 1,
        name: "Terminal",
        pkg_id: "com.omnix.terminal",
        payload: &[],
    },
    OmxApp {
        num_id: 2,
        name: "Explorer",
        pkg_id: "com.omnix.explorer",
        payload: &[],
    },
    OmxApp {
        num_id: 3,
        name: "Settings",
        pkg_id: "com.omnix.settings",
        payload: &[],
    },
];

// 3. Přidána funkce get_default_apps(), kterou volá system_ui.rs
pub fn get_default_apps() -> [OmxApp<'static>; APP_COUNT] {
    DEFAULT_APPS
}

// Parser komunitních balíčků (vrací nyní OmxApp)
pub fn parse_package(data: &[u8]) -> Option<OmxApp> {
    if data.len() < 7 { return None; }
    if &data[0..4] != b"OMX!" { return None; }
    
    let num_id = data[4];
    let name_len = data[5] as usize;
    let pkg_id_len = data[6] as usize;

    let mut offset = 7;
    if data.len() < offset + name_len + pkg_id_len { return None; }

    let name = str::from_utf8(&data[offset..offset + name_len]).unwrap_or("Unknown");
    offset += name_len;
    
    let pkg_id = str::from_utf8(&data[offset..offset + pkg_id_len]).unwrap_or("com.unknown");
    offset += pkg_id_len;

    Some(OmxApp {
        num_id,
        name,
        pkg_id,
        payload: &data[offset..],
    })
}
