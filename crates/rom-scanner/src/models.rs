use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    /// English (Default)
    #[default]
    #[serde(rename = "en", alias = "en-US", alias = "en_US", alias = "en-GB")]
    En,
    /// Portuguese (Brazil)
    #[serde(rename = "pt-BR", alias = "pt", alias = "pt_BR", alias = "pt-br")]
    PtBr,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::PtBr => "pt-BR",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::PtBr => "Português (Brasil)",
        }
    }

    pub fn parse_flexible(s: &str) -> Option<Self> {
        let s_lower = s.trim().to_lowercase().replace('_', "-");
        match s_lower.as_str() {
            "en" | "en-us" | "en-gb" | "english" => Some(Language::En),
            "pt" | "pt-br" | "ptbr" | "portugues" | "portuguese" => Some(Language::PtBr),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LaunchBehavior {
    /// Oculta/minimiza o launcher ao iniciar o jogo e restaura em tela cheia com foco imediato ao fechar (Padrão Console)
    #[default]
    HideAndRestoreFullscreen,
    /// Mantém o launcher aberto em segundo plano
    KeepOpen,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArtSet {
    /// Capa vertical (600x900)
    pub cover: Option<String>,
    /// Imagem de fundo/banner horizontal (1920x620)
    pub hero: Option<String>,
    /// Logo com transparência (PNG)
    pub logo: Option<String>,
    /// Imagem horizontal de biblioteca / wide (460x215)
    pub wide: Option<String>,
    /// Ícone executável (.ico ou .png 256x256)
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreInfo {
    pub name: String,
    pub label: String,
    pub id: String,
    pub namespace: Option<String>,
    pub app_name: Option<String>,
    pub url: Option<String>,
    pub needs_launcher: bool,
    pub launcher_exe: Option<String>,
    pub args: Option<String>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub uuid: Uuid,
    pub title: String,
    pub original_title: Option<String>,
    pub path: String,
    pub platform: String,
    pub platform_name: String,
    pub serial: Option<String>,
    pub disc_id: Option<String>,
    pub steam_app_id: Option<u32>,
    pub shortcut_id: Option<u32>,
    pub store: Option<StoreInfo>,
    pub art: ArtSet,
    pub launch_behavior: LaunchBehavior,
    pub custom_exe: Option<String>,
    pub custom_args: Option<String>,
    pub playtime_seconds: u64,
    pub last_played_at: Option<String>,
    pub hidden: bool,
    pub favorite: bool,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Game {
    pub fn new(
        title: impl Into<String>,
        path: impl Into<String>,
        platform: impl Into<String>,
        platform_name: impl Into<String>,
    ) -> Self {
        let uuid = Uuid::new_v4();
        let path_str = path.into();
        let title_str = title.into();
        Self {
            id: uuid.to_string(),
            uuid,
            title: title_str,
            original_title: None,
            path: path_str,
            platform: platform.into(),
            platform_name: platform_name.into(),
            serial: None,
            disc_id: None,
            steam_app_id: None,
            shortcut_id: None,
            store: None,
            art: ArtSet::default(),
            launch_behavior: LaunchBehavior::default(),
            custom_exe: None,
            custom_args: None,
            playtime_seconds: 0,
            last_played_at: None,
            hidden: false,
            favorite: false,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}
