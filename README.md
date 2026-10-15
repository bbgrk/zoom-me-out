# Auto Zoom Out

Zoom Me Out is a small Raidcore Nexus addon for Guild Wars 2 that presses your Camera Zoom Out key for you at a steady interval, with afk check. That is all it does.

## Features

- Presses the game's Camera Zoom Out keybind repeatedly, at an interval you choose (50 to 1000 ms)
- Checks whether the player has moved within the last 30 seconds. While you keep moving, the zoom out keeps running. If you stand still for 30 seconds, it pauses, and it resumes as soon as you move again.
- Enable/disable with one checkbox, disabled by default
- Settings are saved to a config file and restored on the next start
- The pressed action is fixed to Camera Zoom Out and cannot be changed to anything else

## Requirements

Raidcore Nexus installed

## Install

- Download `zoom_me_out.dll` (or build it yourself).
- Copy it into the addons folder of your Guild Wars 2 installation, for example `C:\Program Files\Guild Wars 2\addons\`.
- Start the game, or load the addon from the Nexus addon menu.
- In Guild Wars 2, open the options, go to Control Options and bind Camera Zoom Out to the key you want (for example Page Down). The addon presses whatever key is bound there.

## Usage

Open the Nexus addon menu and go to the options for **Zoom Me Out**:

| Setting | Description |
| --- | --- |
| Enabled | Turns the automatic pressing on or off. Off by default. |
| Interval (ms) | Time between presses, from 50 to 1000. Default is 100. |

## Configuration file

Settings are written to `config.json` inside the addon's own folder under `addons\` (`zoom-me-out`). The file is created the first time you change a setting. Example:

```json
{
  "enabled": true,
  "interval_ms": 100
}
```

Invalid or out-of-range values are corrected when the addon loads.

## Build from source

You need Windows and a recent Rust toolchain (1.85 or newer, edition 2024).

```
git clone <this repository>
cd auto-zoom-out
cargo build --release
```

The addon is built to `target\release\zoom-me-out.dll`. Cargo turns the hyphens in the project name into underscores for the file name.

## Disclaimer

This project is not affiliated with or endorsed by ArenaNet or Raidcore. It sends only the Camera Zoom Out keybind through the Nexus API. Use it at your own risk and make sure it fits the game's rules for your situation.

## License

Unlicense

## Addon Policy

- https://raidcore.gg/gw2/addon-policy
- https://help.guildwars2.com/hc/en-us/articles/360013762153-Policy-Macros-and-Macro-Use
- https://help.guildwars2.com/hc/en-us/articles/360013625034-Policy-Third-Party-Programs

