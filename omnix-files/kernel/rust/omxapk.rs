#[derive(Copy, Clone)]
pub struct OmxApp<'a> {
    pub id: u8,
    pub name: &'a str,
    pub payload: &'a [u8],
}

pub const APP_COUNT: usize = 5;

pub fn parse_package(data: &'static [u8]) -> Option<OmxApp<'static>> {
    if data.len() < 6 || &data[0..4] != b"OMX!" {
        return None;
    }

    let id = data[4];
    let name_len = data[5] as usize;

    if data.len() < 6 + name_len {
        return None;
    }

    let name_bytes = &data[6..6 + name_len];
    let name = core::str::from_utf8(name_bytes).unwrap_or("Unknown");
    let payload = &data[6 + name_len..];

    Some(OmxApp { id, name, payload })
}

pub fn get_default_apps() -> [OmxApp<'static>; APP_COUNT] {
    [
        parse_package(crate::TERMINAL_APK).unwrap(),
        parse_package(crate::EXPLORER_APK).unwrap(),
        parse_package(crate::SETTINGS_APK).unwrap(),
    ]
}
