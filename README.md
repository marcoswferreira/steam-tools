# Steam Tools

High-performance Non-Steam Game Importer, Steam Grid Artwork Manager, Emulator Launcher Resolver, and CDP Achievement Toaster for Steam Deck & Desktop.

## Features

- **Deep ROM & Executable Scanner**: Instant header inspection for PS1, PS2, PS3, GameCube, Wii, Switch, N64, GBA, NDS, SNES, Genesis, and PC Windows executables.
- **Automated Emulator Resolution**: Auto-locates PCSX2, RPCS3, Dolphin, DuckStation, RetroArch, Ryujinx, Xenia and crafts fullscreen arguments.
- **Deterministic 32-bit CRC Steam AppIDs**: Matches Valve's exact hashing formula so custom grid artwork stays persistent.
- **Steam Grid Artwork Engine**: Auto-detects local cover, hero, logo, wide banner, and icons adjacent to game files and copies them to `userdata/<id>/config/grid/`.
- **Safe `shortcuts.vdf` Synchronization**: Atomic backups (`shortcuts.vdf.bak`), lock removal, and conflict avoidance when Steam is running.
- **CDP Achievement Bridge**: Connects to the Steam client UI via Chrome DevTools Protocol (`127.0.0.1:8080`) to trigger native achievement notifications for emulators.

## License

GPL-3.0-or-later
