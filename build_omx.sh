#!/bin/bash

# Nastavení složek
APPS_DIR="omnix-files/default-apps"
BOOT_DIR="omnix-files/boot/apps"
MANIFEST="$APPS_DIR/manifest"

echo "======================================"
echo "    OmniX OS - .omxapk Builder 🚀     "
echo "======================================"

# Vytvoření cílové složky v boot, pokud chybí
mkdir -p "$BOOT_DIR"

if [ ! -f "$MANIFEST" ]; then
    echo "❌ CHYBA: Soubor $MANIFEST nebyl nalezen!"
    exit 1
fi

# Čtení manifestu řádek po řádku
while IFS=',' read -r app_id app_name app_source || [ -n "$app_id" ]; do
    # Přeskočení prázdných řádků a komentářů (začínajících na #)
    [[ "$app_id" =~ ^#.*$ ]] || [ -z "$app_id" ] && continue

    echo "⚙️  Sestavuji aplikaci: $app_name (ID: $app_id)"

    TARGET_FILE="$BOOT_DIR/$app_name.omxapk"
    SOURCE_PATH="$APPS_DIR/$app_source"

    # Zkontrolujeme, jestli vůbec máš zdroják
    if [ ! -f "$SOURCE_PATH" ]; then
        echo "   ⚠️ Varování: Zdrojový soubor $SOURCE_PATH neexistuje. Udělám prázdný balíček."
        touch "$SOURCE_PATH"
    fi

    # --------------------------------------------------
    # TVORBA BINÁRNÍHO FORMÁTU .omxapk
    # 1. Magická hlavička: 4 byty "OMX!" (Aby jádro poznalo, že to je tvůj formát)
    printf "OMX!" > "$TARGET_FILE"
    
    # 2. ID Aplikace (1 byte v HEX)
    printf "\\x$(printf '%02x' $app_id)" >> "$TARGET_FILE"
    
    # 3. Délka názvu (1 byte)
    NAME_LEN=${#app_name}
    printf "\\x$(printf '%02x' $NAME_LEN)" >> "$TARGET_FILE"
    
    # 4. Samotný název aplikace (Terminal, atd.)
    printf "%s" "$app_name" >> "$TARGET_FILE"
    
    # 5. Samotná data (zdrojový kód nebo binárka)
    cat "$SOURCE_PATH" >> "$TARGET_FILE"
    # --------------------------------------------------

    echo "   ✅ Úspěch: Vytvořeno $TARGET_FILE ($(stat -c%s "$TARGET_FILE" 2>/dev/null || stat -f%z "$TARGET_FILE") bytů)"

done < "$MANIFEST"

echo "======================================"
echo "🎉 Všechny aplikace zkompilovány do boot/apps!"
echo "======================================"
