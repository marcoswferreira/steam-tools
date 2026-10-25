use crate::disc::inspect_disc;
use crate::headers::read_cartridge_header;
use crate::models::Game;
use crate::pe::find_steam_app_id_near;
use crate::platforms::{guess_platform_by_ext, guess_platform_by_folder_name};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ScannerOptions {
    pub search_roots: Vec<PathBuf>,
    pub max_depth: usize,
}

impl Default for ScannerOptions {
    fn default() -> Self {
        Self {
            search_roots: Vec::new(),
            max_depth: 5,
        }
    }
}

/// Searches local directories adjacent to a game or ROM for cover art, heroes, logos and icons
pub fn find_local_game_artwork(path: &Path) -> crate::models::ArtSet {
    let mut art = crate::models::ArtSet::default();
    let parent = match path.parent() {
        Some(p) => p,
        None => return art,
    };
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    // 1. Cover / Poster (Vertical)
    let cover_candidates = [
        format!("{}.png", stem),
        format!("{}.jpg", stem),
        format!("{}_cover.png", stem),
        format!("{}_cover.jpg", stem),
        format!("{}_poster.png", stem),
        "cover.png".to_string(),
        "cover.jpg".to_string(),
        "poster.png".to_string(),
        "poster.jpg".to_string(),
        "folder.jpg".to_string(),
        "boxart.png".to_string(),
    ];
    for name in &cover_candidates {
        let p = parent.join(name);
        if p.is_file() {
            art.cover = Some(p.to_string_lossy().to_string());
            break;
        }
    }

    // 2. Hero / Background (Horizontal widescreen)
    let hero_candidates = [
        format!("{}_hero.png", stem),
        format!("{}_hero.jpg", stem),
        format!("{}_background.jpg", stem),
        "hero.png".to_string(),
        "hero.jpg".to_string(),
        "background.png".to_string(),
        "background.jpg".to_string(),
        "banner.png".to_string(),
    ];
    for name in &hero_candidates {
        let p = parent.join(name);
        if p.is_file() {
            art.hero = Some(p.to_string_lossy().to_string());
            break;
        }
    }

    // 3. Logo (Transparent PNG)
    let logo_candidates = [
        format!("{}_logo.png", stem),
        "logo.png".to_string(),
        "title.png".to_string(),
    ];
    for name in &logo_candidates {
        let p = parent.join(name);
        if p.is_file() {
            art.logo = Some(p.to_string_lossy().to_string());
            break;
        }
    }

    // 4. Icon (.ico or .png)
    let icon_candidates = [
        format!("{}_icon.ico", stem),
        format!("{}_icon.png", stem),
        format!("{}.ico", stem),
        "icon.ico".to_string(),
        "icon.png".to_string(),
    ];
    for name in &icon_candidates {
        let p = parent.join(name);
        if p.is_file() {
            art.icon = Some(p.to_string_lossy().to_string());
            break;
        }
    }

    if art.icon.is_none() && path.extension().map(|e| e == "exe").unwrap_or(false) {
        art.icon = Some(path.to_string_lossy().to_string());
    }

    art
}

pub fn scan_path_for_game(path: &Path) -> Option<Game> {
    if !path.is_file() {
        return None;
    }

    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    let file_stem = path.file_stem()?.to_str()?.to_string();

    // 1. Tentar detectar cabeçalho de cartucho (GBA, NDS, GB, N64, Genesis, SNES, NES)
    if let Some(header) = read_cartridge_header(path, &ext) {
        let display_title = if !header.title.is_empty() {
            header.title
        } else {
            file_stem.clone()
        };
        let mut game = Game::new(
            display_title,
            path.to_string_lossy().to_string(),
            header.platform,
            header.platform.to_uppercase(),
        );
        if !header.serial.is_empty() {
            game.serial = Some(header.serial);
        }
        game.art = find_local_game_artwork(path);
        return Some(game);
    }

    // 2. Tentar detectar imagem de disco (ISO, BIN, CUE, CHD, CSO)
    if ["iso", "bin", "cue", "chd", "cso"].contains(&ext.as_str()) {
        if let Some(disc) = inspect_disc(path) {
            let title = disc.title.unwrap_or_else(|| file_stem.clone());
            let mut game = Game::new(
                title,
                path.to_string_lossy().to_string(),
                disc.platform,
                disc.platform.to_uppercase(),
            );
            game.serial = disc.serial;
            game.disc_id = Some(disc.volume_id);
            game.art = find_local_game_artwork(path);
            return Some(game);
        }
    }

    // 3. Tentar detectar por extensão e pasta do sistema
    if let Some(platform_def) = guess_platform_by_ext(&ext) {
        let mut game = Game::new(
            file_stem,
            path.to_string_lossy().to_string(),
            platform_def.key,
            platform_def.name,
        );

        if platform_def.key == "pc" {
            if let Some(steam_ev) = find_steam_app_id_near(path) {
                game.steam_app_id = Some(steam_ev.app_id);
            }
        }
        game.art = find_local_game_artwork(path);
        return Some(game);
    }

    // 4. Se estiver dentro de uma pasta com o nome de uma plataforma conhecida
    if let Some(parent) = path.parent() {
        if let Some(folder_name) = parent.file_name().and_then(|f| f.to_str()) {
            if let Some(platform_def) = guess_platform_by_folder_name(folder_name) {
                let mut game = Game::new(
                    file_stem,
                    path.to_string_lossy().to_string(),
                    platform_def.key,
                    platform_def.name,
                );
                game.art = find_local_game_artwork(path);
                return Some(game);
            }
        }
    }

    None
}

pub fn scan_roots_parallel(roots: &[PathBuf], max_depth: usize) -> Vec<Game> {
    roots
        .par_iter()
        .flat_map(|root| {
            if !root.exists() {
                return Vec::new();
            }
            WalkDir::new(root)
                .max_depth(max_depth)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .par_bridge()
                .filter_map(|entry| scan_path_for_game(entry.path()))
                .collect::<Vec<Game>>()
        })
        .collect()
}
