# romantic installer

Installs romantic into Discord, Discord PTB or Discord Canary. Open `romantic-installer.exe`, pick your Discord, hit Install. Repair re-applies it after a Discord update, Uninstall puts Discord back.

Command line: `--list`, `--install`, `--repair`, `--uninstall`, and optionally `--branch Discord|DiscordPTB|DiscordCanary`.

To build it, copy `dist/desktop/{package.json,patcher.js,preload.js,renderer.js,renderer.css}` from the mod build into `assets/`, then run `cargo build --release`.

GPL-3.0, see `LICENSE`.
