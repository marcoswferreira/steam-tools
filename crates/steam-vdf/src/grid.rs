use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SteamGridManager {
    pub grid_dir: PathBuf,
}

impl SteamGridManager {
    pub fn new(grid_dir: PathBuf) -> Self {
        Self { grid_dir }
    }

    pub fn cover_path(&self, shortcut_id: u32) -> PathBuf {
        self.grid_dir.join(format!("{}p.png", shortcut_id))
    }

    pub fn hero_path(&self, shortcut_id: u32) -> PathBuf {
        self.grid_dir.join(format!("{}_hero.png", shortcut_id))
    }

    pub fn logo_path(&self, shortcut_id: u32) -> PathBuf {
        self.grid_dir.join(format!("{}_logo.png", shortcut_id))
    }

    pub fn wide_path(&self, shortcut_id: u32) -> PathBuf {
        self.grid_dir.join(format!("{}.png", shortcut_id))
    }

    pub fn icon_path(&self, shortcut_id: u32) -> PathBuf {
        self.grid_dir.join(format!("{}_icon.ico", shortcut_id))
    }

    pub fn copy_art_file(
        &self,
        shortcut_id: u32,
        art_type: &str,
        source_path: &std::path::Path,
    ) -> std::io::Result<PathBuf> {
        if !source_path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source art not found: {}", source_path.display()),
            ));
        }
        fs::create_dir_all(&self.grid_dir)?;
        let target = match art_type {
            "cover" => self.cover_path(shortcut_id),
            "hero" => self.hero_path(shortcut_id),
            "logo" => self.logo_path(shortcut_id),
            "wide" => self.wide_path(shortcut_id),
            "icon" => self.icon_path(shortcut_id),
            _ => self.cover_path(shortcut_id),
        };
        fs::copy(source_path, &target)?;
        Ok(target)
    }

    pub fn save_art(
        &self,
        shortcut_id: u32,
        art_type: &str,
        bytes: &[u8],
    ) -> std::io::Result<PathBuf> {
        fs::create_dir_all(&self.grid_dir)?;
        let target = match art_type {
            "cover" => self.cover_path(shortcut_id),
            "hero" => self.hero_path(shortcut_id),
            "logo" => self.logo_path(shortcut_id),
            "wide" => self.wide_path(shortcut_id),
            "icon" => self.icon_path(shortcut_id),
            _ => self.cover_path(shortcut_id),
        };
        fs::write(&target, bytes)?;
        Ok(target)
    }
}
