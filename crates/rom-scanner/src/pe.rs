use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamEvidence {
    pub app_id: u32,
    pub source: String,
}

pub fn detect_steam_app_id_in_dir(dir: &Path) -> Option<SteamEvidence> {
    // 1. steam_appid.txt
    for name in &["steam_appid.txt", "steam_settings/steam_appid.txt"] {
        let file = dir.join(name);
        if file.is_file() {
            if let Ok(text) = fs::read_to_string(&file) {
                let trimmed = text.trim();
                if let Ok(id) = trimmed.parse::<u32>() {
                    if id > 0 {
                        return Some(SteamEvidence {
                            app_id: id,
                            source: name.to_string(),
                        });
                    }
                }
            }
        }
    }

    // 2. Arquivos ini conhecidos de emuladores de Steam / loaders
    let ini_files = &[
        "steam_emu.ini",
        "ColdClientLoader.ini",
        "cream_api.ini",
        "OnlineFix.ini",
        "ALI213.ini",
        "SmartSteamEmu.ini",
        "CODEX.ini",
        "RUNE.ini",
        "steam_settings/configs.app.ini",
        "steam_settings/configs.main.ini",
    ];

    let re_appid =
        regex::Regex::new(r#"(?im)^\s*(?:Real)?App[_ ]?I[Dd]\s*=\s*"?([1-9]\d{0,9})"?\s*$"#)
            .ok()?;

    for name in ini_files {
        let file = dir.join(name);
        if file.is_file() {
            if let Ok(text) = fs::read_to_string(&file) {
                if let Some(caps) = re_appid.captures(&text) {
                    if let Some(m) = caps.get(1) {
                        if let Ok(id) = m.as_str().parse::<u32>() {
                            return Some(SteamEvidence {
                                app_id: id,
                                source: name.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn find_steam_app_id_near(file_path: &Path) -> Option<SteamEvidence> {
    let mut current = file_path.parent()?;
    for _ in 0..4 {
        if let Some(ev) = detect_steam_app_id_in_dir(current) {
            return Some(ev);
        }
        current = current.parent()?;
    }
    None
}
