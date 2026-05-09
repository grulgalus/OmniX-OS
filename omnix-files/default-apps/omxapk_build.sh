#!/bin/bash

# Zkontrolujeme, jestli jsi nám dal jméno složky
if [ -z "$1" ]; then
    echo "❌ Chyba: Musíš zadat složku aplikace!"
    echo "Příklad: bash omxapk_build.sh default-apps/Terminal"
    exit 1
fi

APP_FOLDER="$1"
OUT_DIR="omnix-files/boot/apps"

if [ ! -d "$APP_FOLDER" ]; then
    echo "❌ Složka '$APP_FOLDER' neexistuje!"
    exit 1
fi

mkdir -p "$OUT_DIR"

# Tady posíláme cestu ke složce do Pythonu jako argument 
python3 << 'EOF' "$APP_FOLDER" "$OUT_DIR"
import os, sys, json, struct

# Proměnné od Bashe (z argumentů na řádku nahoře)
app_path = sys.argv[1]
out_dir = sys.argv[2]
app_folder_name = os.path.basename(os.path.normpath(app_path))

json_path = os.path.join(app_path, 'app.json')
if not os.path.exists(json_path):
    print(f'❌ Chyba: Ve složce chybí {json_path}!')
    sys.exit(1)

try:
    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
except Exception as e:
    print(f'❌ Chyba ve formátu app.json: {e}')
    sys.exit(1)

# Bezpečné čtení stringů
num_id = data.get('num-id', 255)
name = data.get('package-name', app_folder_name).encode('utf-8')
pkg_id = data.get('package-id', f'com.omnix.{app_folder_name.lower()}').encode('utf-8')
build_cmd = data.get('build-cmd', '')
payload_file = data.get('payload-file', '')

print(f'\n📦 Buduji aplikaci z: {app_path}')
print(f'   🏷️  Jméno: {data.get("package-name", app_folder_name)}')

if build_cmd:
    print(f'   ⚙️  Spouštím kompilátor: {build_cmd}')
    result = os.system(f'cd {app_path} && {build_cmd}')
    if result != 0:
        print('   ❌ Kompilace zdrojáků selhala!')
        sys.exit(1)

payload_path = os.path.join(app_path, payload_file)
if not os.path.exists(payload_path):
    print(f'   ❌ Nenalezen zkompilovaný soubor / zdroják: {payload_path}')
    sys.exit(1)

with open(payload_path, 'rb') as f:
    payload = f.read()

out_file = os.path.join(out_dir, f'{app_folder_name.lower()}.omxapk')

with open(out_file, 'wb') as f:
    f.write(b'OMX!')
    f.write(struct.pack('B', num_id))
    f.write(struct.pack('B', len(name)))
    f.write(struct.pack('B', len(pkg_id)))
    f.write(name)
    f.write(pkg_id)
    f.write(payload)

print(f'   ✅ HOTOVO! -> {out_file}')
EOF
