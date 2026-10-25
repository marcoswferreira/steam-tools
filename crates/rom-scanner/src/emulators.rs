use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct EmulatorProfile {
    pub key: &'static str,
    pub name: &'static str,
    pub files: &'static [&'static str],
    pub default_args: &'static str,
    pub fullscreen_args: &'static str,
    pub download_url: Option<&'static str>,
}

pub static EMULATORS: &[EmulatorProfile] = &[
    EmulatorProfile {
        key: "psp",
        name: "PPSSPP",
        files: &[
            "PPSSPPWindows64.exe",
            "PPSSPPWindows.exe",
            "PPSSPPDL.exe",
            "ppsspp",
        ],
        default_args: r#""{game}""#,
        fullscreen_args: r#"--fullscreen "{game}""#,
        download_url: Some("https://www.ppsspp.org/"),
    },
    EmulatorProfile {
        key: "ps2",
        name: "PCSX2",
        files: &[
            "pcsx2-qtx64.exe",
            "pcsx2-qt.exe",
            "pcsx2x64.exe",
            "pcsx2.exe",
            "pcsx2",
        ],
        default_args: r#"-batch "{game}""#,
        fullscreen_args: r#"-batch -fullscreen "{game}""#,
        download_url: Some("https://pcsx2.net/"),
    },
    EmulatorProfile {
        key: "ps1",
        name: "DuckStation",
        files: &[
            "duckstation-qt-x64-ReleaseLTCG.exe",
            "duckstation-nogui-x64-ReleaseLTCG.exe",
            "duckstation-qt.exe",
            "duckstation",
        ],
        default_args: r#"-batch "{game}""#,
        fullscreen_args: r#"-batch -fullscreen "{game}""#,
        download_url: Some("https://www.duckstation.org/"),
    },
    EmulatorProfile {
        key: "ps3",
        name: "RPCS3",
        files: &["rpcs3.exe", "rpcs3"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"--fullscreen "{game}""#,
        download_url: Some("https://rpcs3.net/"),
    },
    EmulatorProfile {
        key: "dolphin",
        name: "Dolphin",
        files: &["Dolphin.exe", "dolphin-emu"],
        default_args: r#"-b -e "{game}""#,
        fullscreen_args: r#"-b --batch -e "{game}""#,
        download_url: Some("https://dolphin-emu.org/"),
    },
    EmulatorProfile {
        key: "cemu",
        name: "Cemu",
        files: &["Cemu.exe", "cemu"],
        default_args: r#"-g "{game}""#,
        fullscreen_args: r#"-f -g "{game}""#,
        download_url: Some("https://cemu.info/"),
    },
    EmulatorProfile {
        key: "switch",
        name: "Ryujinx / Yuzu / Citron",
        files: &[
            "Ryujinx.exe",
            "yuzu.exe",
            "citron.exe",
            "suyu.exe",
            "eden.exe",
            "sudachi.exe",
            "Ryujinx",
            "yuzu",
        ],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f -g "{game}""#,
        download_url: None,
    },
    EmulatorProfile {
        key: "mgba",
        name: "mGBA",
        files: &["mGBA.exe", "mgba"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f "{game}""#,
        download_url: Some("https://mgba.io/"),
    },
    EmulatorProfile {
        key: "melonds",
        name: "melonDS",
        files: &["melonDS.exe", "melonds"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f "{game}""#,
        download_url: Some("https://melonds.kuribo64.net/"),
    },
    EmulatorProfile {
        key: "snes9x",
        name: "Snes9x",
        files: &["snes9x-x64.exe", "snes9x.exe", "snes9x"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-fullscreen "{game}""#,
        download_url: Some("https://www.snes9x.com/"),
    },
    EmulatorProfile {
        key: "xenia",
        name: "Xenia",
        files: &["xenia.exe", "xenia_canary.exe", "xenia"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"--fullscreen "{game}""#,
        download_url: Some("https://xenia.jp/"),
    },
    EmulatorProfile {
        key: "retroarch",
        name: "RetroArch",
        files: &["retroarch.exe", "retroarch"],
        default_args: r#"-L {core} "{game}""#,
        fullscreen_args: r#"-f -L {core} "{game}""#,
        download_url: Some("https://www.retroarch.com/"),
    },
];

pub fn find_emulator_by_key(key: &str) -> Option<&'static EmulatorProfile> {
    EMULATORS.iter().find(|e| e.key == key)
}

pub fn guess_profile_from_exe(exe_path: &Path) -> Option<&'static EmulatorProfile> {
    let file_name = exe_path.file_name()?.to_str()?.to_ascii_lowercase();
    EMULATORS
        .iter()
        .find(|e| e.files.iter().any(|f| f.eq_ignore_ascii_case(&file_name)))
}

pub fn detected_arguments(profile_key: &str, _exe_path: &Path) -> String {
    if let Some(emu) = find_emulator_by_key(profile_key) {
        emu.fullscreen_args.to_string()
    } else {
        r#""{game}""#.to_string()
    }
}

pub fn standard_discovery_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(home) = dirs::home_dir() {
        roots.push(home.join("Emulators"));
        roots.push(home.join("Emuladores"));
        roots.push(home.join("Games"));
        roots.push(home.join("Jogos"));
        roots.push(home.join("AppData/Local/Programs"));
    }
    #[cfg(target_os = "windows")]
    {
        roots.push(PathBuf::from(r"C:\Emulators"));
        roots.push(PathBuf::from(r"C:\Emuladores"));
        roots.push(PathBuf::from(r"C:\Jogos"));
        roots.push(PathBuf::from(r"C:\Games"));
        roots.push(PathBuf::from(r"D:\Emulators"));
        roots.push(PathBuf::from(r"D:\Emuladores"));
        roots.push(PathBuf::from(r"D:\Games"));
        roots.push(PathBuf::from(r"D:\Jogos"));
        if let Ok(pf) = std::env::var("ProgramFiles") {
            roots.push(PathBuf::from(pf));
        }
        if let Ok(pfx86) = std::env::var("ProgramFiles(x86)") {
            roots.push(PathBuf::from(pfx86));
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            roots.push(PathBuf::from(local).join("Programs"));
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs::home_dir() {
            roots.push(home.join(".local/share"));
            roots.push(home.join("Applications"));
            roots.push(home.join(".var/app"));
        }
        roots.push(PathBuf::from("/usr/bin"));
        roots.push(PathBuf::from("/usr/local/bin"));
    }
    roots
}

/// Automatically searches standard discovery roots and system PATH for an emulator binary
pub fn auto_detect_emulator_path(profile: &EmulatorProfile) -> Option<PathBuf> {
    for root in standard_discovery_roots() {
        for file in profile.files {
            let candidate = root.join(file);
            if candidate.is_file() {
                return Some(candidate);
            }
            let sub = root.join(profile.name).join(file);
            if sub.is_file() {
                return Some(sub);
            }
            let sub_key = root.join(profile.key).join(file);
            if sub_key.is_file() {
                return Some(sub_key);
            }
        }
    }

    if let Ok(path_var) = std::env::var("PATH") {
        for p in std::env::split_paths(&path_var) {
            for file in profile.files {
                let candidate = p.join(file);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

fn get_parent_dir_string(path_str: &str) -> String {
    let normalized = path_str.replace('\\', "/");
    if let Some(pos) = normalized.rfind('/') {
        if pos == 0 {
            "/".to_string()
        } else {
            path_str[..pos].to_string()
        }
    } else {
        String::new()
    }
}

/// Resolves the launch command: (executable_path, launch_arguments, working_directory)
pub fn resolve_game_launch_command(
    game: &crate::models::Game,
    custom_emulator_map: &std::collections::HashMap<String, String>,
) -> (String, String, String) {
    if let Some(custom_exe) = &game.custom_exe {
        if !custom_exe.trim().is_empty() {
            let start_dir = get_parent_dir_string(custom_exe);
            let args = game.custom_args.clone().unwrap_or_default();
            return (custom_exe.clone(), args, start_dir);
        }
    }

    let is_native_pe = game.path.to_ascii_lowercase().ends_with(".exe");
    if is_native_pe {
        let start_dir = get_parent_dir_string(&game.path);
        let args = game.custom_args.clone().unwrap_or_default();
        return (game.path.clone(), args, start_dir);
    }

    let platform_def = crate::platforms::find_platform_by_key(&game.platform);
    let profile_key = platform_def
        .and_then(|p| p.default_profiles.first().copied())
        .unwrap_or("retroarch");

    let profile = find_emulator_by_key(profile_key);

    let emu_exe_opt = custom_emulator_map
        .get(profile_key)
        .map(PathBuf::from)
        .or_else(|| profile.and_then(auto_detect_emulator_path));

    if let Some(emu_path) = emu_exe_opt {
        let exe_str = emu_path.to_string_lossy().to_string();
        let start_dir = get_parent_dir_string(&exe_str);

        let raw_args = if let Some(custom) = &game.custom_args {
            if !custom.trim().is_empty() {
                custom.clone()
            } else if let Some(prof) = profile {
                prof.fullscreen_args.to_string()
            } else {
                r#""{game}""#.to_string()
            }
        } else if let Some(prof) = profile {
            prof.fullscreen_args.to_string()
        } else {
            r#""{game}""#.to_string()
        };

        let launch_options = if raw_args.contains("{game}") {
            raw_args.replace("{game}", &game.path)
        } else {
            format!("{} \"{}\"", raw_args, game.path)
        };

        (exe_str, launch_options.trim().to_string(), start_dir)
    } else {
        let start_dir = get_parent_dir_string(&game.path);
        let args = game.custom_args.clone().unwrap_or_default();
        (game.path.clone(), args, start_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Game;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_game_launch_command_native_pe() {
        let game = Game::new("Hades", r"C:\Games\Hades\Hades.exe", "pc", "PC Windows");
        let map = HashMap::new();
        let (exe, args, dir) = resolve_game_launch_command(&game, &map);
        assert_eq!(exe, r"C:\Games\Hades\Hades.exe");
        assert_eq!(args, "");
        assert_eq!(dir, r"C:\Games\Hades");
    }

    #[test]
    fn test_resolve_game_launch_command_with_custom_exe() {
        let mut game = Game::new(
            "GTA Vice City",
            r"D:\Roms\ps2\gtavc.iso",
            "ps2",
            "PlayStation 2",
        );
        game.custom_exe = Some(r"D:\Emus\custom_pcsx2.exe".to_string());
        game.custom_args = Some("-fullscreen \"{game}\"".to_string());

        let map = HashMap::new();
        let (exe, args, dir) = resolve_game_launch_command(&game, &map);
        assert_eq!(exe, r"D:\Emus\custom_pcsx2.exe");
        assert_eq!(args, "-fullscreen \"{game}\"");
        assert_eq!(dir, r"D:\Emus");
    }

    #[test]
    fn test_resolve_game_launch_command_with_emulator_map() {
        let game = Game::new(
            "Grand Theft Auto 2",
            r"D:\Roms\ps2\gta2.chd",
            "ps2",
            "PlayStation 2",
        );
        let mut map = HashMap::new();
        map.insert(
            "ps2".to_string(),
            r"C:\Emulators\PCSX2\pcsx2-qt.exe".to_string(),
        );

        let (exe, args, dir) = resolve_game_launch_command(&game, &map);
        assert_eq!(exe, r"C:\Emulators\PCSX2\pcsx2-qt.exe");
        assert!(args.contains(r"D:\Roms\ps2\gta2.chd"));
        assert!(args.contains("-batch"));
        assert_eq!(dir, r"C:\Emulators\PCSX2");
    }

    #[test]
    fn test_find_local_artwork() {
        let dir = tempfile::tempdir().unwrap();
        let rom = dir.path().join("pokemon.gba");
        std::fs::write(&rom, b"dummy").unwrap();

        let cover = dir.path().join("pokemon.png");
        std::fs::write(&cover, b"fake png").unwrap();

        let hero = dir.path().join("pokemon_hero.png");
        std::fs::write(&hero, b"fake hero").unwrap();

        let art = crate::scanner::find_local_game_artwork(&rom);
        assert_eq!(art.cover, Some(cover.to_string_lossy().to_string()));
        assert_eq!(art.hero, Some(hero.to_string_lossy().to_string()));
    }
}
