pub struct OmxApp<'a> {
    pub id: u8,
    pub name: &'a str,
    pub payload: &'a [u8], // Tady je zabalený ten skutečný kód/text aplikace
}

// Funkce, která rozežere ten tvůj binární formát z bash skriptu!
pub fn parse_package(data: &'static [u8]) -> Option<OmxApp<'static>> {
    // 1. Zkontrolujeme Magic Bytes "OMX!"
    if data.len() < 6 || &data[0..4] != b"OMX!" {
        return None; 
    }

    // 2. Přečteme ID z 5. bajtu (index 4)
    let id = data[4];

    // 3. Přečteme délku názvu z 6. bajtu (index 5)
    let name_len = data[5] as usize;
    if data.len() < 6 + name_len {
        return None;
    }

    // 4. Vytáhneme samotný název aplikace
    let name_bytes = &data[6 .. 6 + name_len];
    let name = core::str::from_utf8(name_bytes).unwrap_or("Neznamy");

    // 5. Zbytek souboru jsou data aplikace!
    let payload = &data[6 + name_len ..];

    Some(OmxApp {
        id,
        name,
        payload,
    })
}
