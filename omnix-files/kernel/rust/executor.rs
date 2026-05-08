use crate::omxapk::OmxApp;

// Pozor, tohle je 'unsafe', protože dáváme plnou kontrolu cizímu kódu!
pub unsafe fn run_omx_app(app: &OmxApp) {
    // 1. Získáme ukazatel (adresu v paměti), kde začíná kód aplikace (payload)
    let code_ptr = app.payload.as_ptr();

    // 2. Přeměníme surová data na spustitelnou funkci! (Function Pointer)
    // V podstatě říkáme: "Od této adresy dál leží strojový kód, připrav se ho spustit."
    let app_function: extern "C" fn() = core::mem::transmute(code_ptr);

    // 3. SKOK DO APLIKACE!
    // Jádro se zde na chvíli zastaví a procesor začne vykonávat kód apky.
    app_function();
}
