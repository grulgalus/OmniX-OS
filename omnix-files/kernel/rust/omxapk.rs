#[derive(Copy, Clone)]
pub struct OmxApp<'a> {
    pub num_id: u8,              // Rychlé číslo pro Window Manager (např. 1, 2, 3)
    pub package_id: &'a str,     // Např. "com.omnix.terminal"
    pub name: &'a str,           // Např. "Terminal"
    pub payload: &'a [u8],       // Samotný kód/data aplikace
}

pub const APP_COUNT: usize = 5;

/// Vylepšený parser, který nerozbije jména a je připraven na tvůj nový formát!
pub fn parse_package(data: &'static [u8]) -> Option<OmxApp<'static>> {
    // Kontrola hlavičky "OMX!" (4 bajty)
    if data.len() < 10 || &data[0..4] != b"OMX!" {
        return None;
    }

    // Pozice 4: Číselné ID pro rychlé přepínání oken
    let num_id = data[4];
    
    // Pozice 5: Délka jména aplikace
    let name_len = data[5] as usize;
    
    // Pozice 6: Délka package-id (com.omnix.něco)
    let pkg_id_len = data[6] as usize;

    let total_header_len = 7 + name_len + pkg_id_len;

    // Bezpečnostní kontrola, jestli soubor není ustřižený
    if data.len() < total_header_len {
        return None;
    }

    // Vytažení Jména
    let name_bytes = &data[7 .. 7 + name_len];
    let name = unsafe { core::str::from_utf8_unchecked(name_bytes) };

    // Vytažení Package ID
    let pkg_id_bytes = &data[7 + name_len .. 7 + name_len + pkg_id_len];
    let package_id = unsafe { core::str::from_utf8_unchecked(pkg_id_bytes) };

    // Zbytek souboru je Payload (tady mohou být zdrojáky nebo ikona v budoucnu)
    let payload = &data[total_header_len..];

    Some(OmxApp {
        num_id,
        package_id,
        name,
        payload,
    })
}

// Inicializace aplikací při bootu systému
pub fn get_default_apps() -> [OmxApp<'static>; APP_COUNT] {
    // Když se něco nepovede načíst, máme záchrannou "Empty" appku
    let fallback = OmxApp { 
        num_id: 255, 
        package_id: "com.omnix.error", 
        name: "Error", 
        payload: &[] 
    };

    [
        parse_package(crate::TERMINAL_APK).unwrap_or(fallback),
        parse_package(crate::EXPLORER_APK).unwrap_or(fallback),
        parse_package(crate::SETTINGS_APK).unwrap_or(fallback),
        fallback, // Tady časem doplníš ty další
        fallback, 
    ]
}
