<img src="installer/assets/brand/romantic.png" width="96" align="left" alt="romantic">

# romantic

Discord, but in love.

<br>

romantic is a Discord client mod with every plugin in one place. 350+ plugins, a clean settings page, and an installer that takes about ten seconds.

## Features

- 350+ plugins built in, nothing extra to download
- Works on Discord, Discord PTB and Discord Canary
- Import your settings from Vencord or Equicord when you install
- Themes and custom CSS
- A heart in your settings, because why not

## Installing / Uninstalling

Grab `romantic-installer.exe` from the [latest release](https://github.com/yearnful/romantic/releases/latest), open it, pick your Discord and hit Install. Discord updates can undo the patch, so if romantic disappears just open the installer and hit Repair. Uninstall puts Discord back how it was.

## Building from source

```
pnpm install
pnpm build
```

That writes `dist/desktop`. To build the installer, copy those files into `installer/assets` and run `cargo build --release` inside `installer`.

## Join the server

Questions, bugs, ideas or just hanging out: https://discord.gg/rNqjKGghBB

## Disclaimer

Client mods go against Discord's Terms of Service. Discord doesn't seem to care much and there are no known cases of people getting banned just for using one, so you're most likely fine. Still, if your account means a lot to you, play it safe and don't use any mod. Also don't post screenshots of romantic in servers where you could get banned for it.

<details>
<summary>License and credits</summary>

romantic is free software under the GPL-3.0-or-later license, see `LICENSE`. It's built on top of earlier open-source Discord mods, [Vencord](https://github.com/Vendicated/Vencord) and [Equicord](https://github.com/Equicord/Equicord), and their copyright notices stay in the source files. Thank you to everyone who made those.

romantic is not affiliated with or endorsed by Discord.

</details>
