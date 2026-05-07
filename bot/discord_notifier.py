import discord
import os
import sys
import asyncio

async def send_notification():
    # Načtení dat z GitHub Actions
    token = os.getenv("DISCORD_TOKEN")
    channel_id = os.getenv("DISCORD_CHANNEL_ID")
    run_number = int(os.getenv("GITHUB_RUN_NUMBER", "0"))
    commit_msg = os.getenv("COMMIT_MESSAGE", "Neznámý commit")
    ping_id = os.getenv("PING_ID", "&1502000912342843523") 

    if not token or not channel_id:
        print("Chyba: Chybí DISCORD_TOKEN nebo DISCORD_CHANNEL_ID v GitHub Secrets!")
        sys.exit(1)

    intents = discord.Intents.default()
    client = discord.Client(intents=intents)

    @client.event
    async def on_ready():
        print(f'Bot přihlášen jako {client.user}')
        try:
            channel = await client.fetch_channel(int(channel_id))
            is_milestone = (run_number > 0) and (run_number % 100 == 0)

            if is_milestone:
                title = f"🏆 HISTORICKÝ MILNÍK: OMNIX OS BUILD {run_number}!"
                color = discord.Color.gold()
                ping_text = f"<@{ping_id}>" if ping_id else "@here"
                content = f"🚨 {ping_text} **DÁMY A PÁNOVÉ, PRÁVĚ JSME DOSÁHLI BUILDU {run_number}!** 🚨\nČas na oslavu! 🍾🥂"
            else:
                title = f"✅ OmniX OS Build {run_number} úspěšný"
                color = discord.Color.green()
                content = ""

            embed = discord.Embed(title=title, description=f"**Commit:** {commit_msg}", color=color)
            embed.set_footer(text="OmniX OS Auto-Notifier")

            # Odeslání zprávy a okamžité vypnutí bota
            await channel.send(content=content, embed=embed)
            print(f"✅ Zpráva pro build {run_number} úspěšně odeslána!")
            await client.close()

        except Exception as e:
            print(f"❌ Nastala chyba při odesílání: {e}")
            await client.close()
            sys.exit(1)

    await client.start(token)

if __name__ == "__main__":
    asyncio.run(send_notification())
