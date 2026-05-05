# 💾 OmniX OS (MS-DOS Edition)

Vítejte v repozitáři **OmniX OS**! Tento projekt začal jako experiment s vlastním linuxovým jádrem, ale nakonec se vyvinul v něco mnohem více retro a epického – operační systém postavený přímo na **oficiálních zdrojových kódech Microsoft MS-DOS**.

Celý vývoj, kompilace a správa tohoto repozitáře probíhá hardcore stylem: čistě na Android tabletu přes Termux! 📱💻

## ✨ O projektu
OmniX OS je oživená legenda. Bere původní, desítky let staré zdrojáky (napsané v Assembly a starém C) a pomocí moderní cloudové automatizace je překládá do funkčního, bootovacího systému. Cílem je mít bleskově rychlý, nenáročný OS, který nastartuje v emulátoru v řádu milisekund.

## 🚀 Jak to funguje (Automatizace)
Abychom nemuseli složitě hledat 40 let staré kompilátory pro Android, využíváme sílu cloudu:
1. Zdrojové kódy MS-DOSu (`.ASM`, `MAKEFILE`, `RUNME.BAT`) jsou uložené v tomto repozitáři.
2. O kompilaci se stará **GitHub Actions** (skripty ve složce `.github/workflows`).
3. Při každé úpravě kódu se na serverech GitHubu automaticky spustí build, který zdrojáky přeloží.
4. Výsledkem je hotový bootovací obraz (`.img` nebo `.iso`), který si stačí stáhnout.

## 🎮 Jak si OmniX OS spustit
Systém je primárně stavěn pro běh v aplikaci **Limbo x86 Emulator** na Androidu.
1. Otevřete záložku **Actions** nahoře v tomto repozitáři.
2. Rozklikněte nejnovější zelený build a stáhněte si z něj výsledný "Artifact" (obraz disku).
3. Otevřete aplikaci Limbo ve svém telefonu/tabletu.
4. Vložte stažený soubor jako *Floppy* (disketu) nebo *Hard Disk*.
5. Nastartujte virtuální stroj a užijte si legendární příkazovou řádku!

## 📜 Historie vývoje
Tento projekt vznikl krví, potem a slzami v terminálu. Máme za sebou tvrdé bitvy s Gitem, včetně:
- Pokusu o upload gigabajtového Arch Linuxu přes mobilní terminál (a následné zjištění, že GitHub má tvrdý limit 100 MB 🛑).
- Cestování v čase přes `git log` a `checkout` pro záchranu smazaných dat.
- Finálního znovuzrození repozitáře do čisté MS-DOS formy.
