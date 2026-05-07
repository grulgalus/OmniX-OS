import os
import sys
import requests

def main():
    # Načtení dat z GitHub Actions
    webhook_url = os.getenv("DISCORD_WEBHOOK")
    run_number = int(os.getenv("GITHUB_RUN_NUMBER", "0"))
    commit_msg = os.getenv("COMMIT_MESSAGE", "Neznámý commit")
    repo = os.getenv("GITHUB_REPOSITORY", "OmniX-OS")
    
    # ID uživatele nebo role, kterou chceš označit (pingnout)
    ping_id = os.getenv("PING_ID", "&1502000912342843523") 

    if not webhook_url:
        print("Chyba: Nenalezen DISCORD_WEBHOOK v tajnostech (Secrets).")
        sys.exit(1)

    # Je to jubilejní build (násobek 100)?
    is_milestone = (run_number > 0) and (run_number % 100 == 0)

    # Sestavení zprávy
    if is_milestone:
        title = f"🏆 HISTORICKÝ MILNÍK: BUILD #{run_number}!"
        color = 0xFFD700  # Zlatá barva
        ping_text = f"<@{ping_id}>" if ping_id else "@here"
        content = f"🚨 {ping_text} **DÁMY A PÁNOVÉ, OMNIX OS PRÁVĚ DOSÁHL BUILDU #{run_number}!** 🚨\nČas otevřít šampaňské! 🍾🥂"
    else:
        title = f"✅ OmniX OS Build #{run_number} úspěšný"
        color = 0x00FF00  # Zelená barva
        content = "" # Běžný build bez pingu

    # Formátování Discord Embedu
    payload = {
        "content": content,
        "embeds": [{
            "title": title,
            "description": f"**Repozitář:** {repo}\n**Commit:** {commit_msg}",
            "color": color,
            "footer": {"text": "OmniX OS Auto-Notifier"}
        }]
    }

    # Odeslání do Discordu
    response = requests.post(webhook_url, json=payload)
    
    if response.status_code == 204:
        print(f"✅ Úspěšně odesláno do Discordu (Build #{run_number})")
    else:
        print(f"❌ Chyba při odesílání: {response.status_code}")

if __name__ == "__main__":
    main()
