use core::str;

// Struktura tvého balíčku pro systém (Start Menu a spouštění)
#[derive(Clone, Copy)]
pub struct OmxApk<'a> {
    pub num_id: u8,
    pub name: &'a str,
    pub pkg_id: &'a str,
    pub payload: &'a [u8],
}

// Natvrdo zapsané defaultní aplikace. Mají prázdný payload,
// protože se nespouští z paměti, ale přímo z jádra (z modulu apps::).
pub const DEFAULT_APPS: [OmxApk<'static>; 3] = [
    OmxApk {
        num_id: 1,
        name: "Terminal",
        pkg_id: "com.omnix.terminal",
        payload: &[],
    },
    OmxApk {
        num_id: 2,
        name: "Explorer",
        pkg_id: "com.omnix.explorer",
        payload: &[],
    },
    OmxApk {
        num_id: 3,
        name: "Settings",
        pkg_id: "com.omnix.settings",
        payload: &[],
    },
];

// Parser pro opravdové .omxapk balíčky (komunitní appky načtené z disku)
// Přesně odpovídá tvému Python builderu (hlavička b'OMX!')
pub fn parse_package(data: &[u8]) -> Option<OmxApk> {
    // Balíček musí mít alespoň 7 bajtů základní hlavičky
    if data.len() < 7 {
        return None;
    }
    
    // Kontrola tvého magického slova
    if &data[0..4] != b"OMX!" {
        return None;
    }
    
    let num_id = data[4];
    let name_len = data[5] as usize;
    let pkg_id_len = data[6] as usize;

    let mut offset = 7;
    
    // Kontrola, jestli soubor není poškozený/useknutý
    if data.len() < offset + name_len + pkg_id_len {
        return None;
    }

    let name_bytes = &data[offset..offset + name_len];
    offset += name_len;
    let pkg_id_bytes = &data[offset..offset + pkg_id_len];
    offset += pkg_id_len;

    // Převod bytů na text (pokud je v tom nesmysl, použije se fallback string)
    let name = str::from_utf8(name_bytes).unwrap_or("Unknown App");
    let pkg_id = str::from_utf8(pkg_id_bytes).unwrap_or("com.unknown");
    
    // Zbytek souboru je samotný zkompilovaný binární kód
    let payload = &data[offset..];

    Some(OmxApk {
        num_id,
        name,
        pkg_id,
        payload,
    })
}
