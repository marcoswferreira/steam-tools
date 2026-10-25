use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
    pub points: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockEvent {
    pub id: String,
    pub game_id: String,
    pub achievement_id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub source: String,
    pub unlocked_at: String,
    pub steam_notified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rpcs3Trophy {
    pub trophy_id: u32,
    pub title: String,
    pub detail: String,
    pub unlocked: bool,
}

pub fn parse_rpcs3_trophy_log_line(line: &str) -> Option<(u32, String)> {
    // Exemplo de log RPCS3: "Trophy unlocked: [NPWR01234_00] Trophy ID 5 (Master of Arms)"
    let re = regex::Regex::new(r#"Trophy unlocked:.*?ID\s+(\d+)\s*\((.+?)\)"#).ok()?;
    let caps = re.captures(line)?;
    let id = caps.get(1)?.as_str().parse::<u32>().ok()?;
    let name = caps.get(2)?.as_str().to_string();
    Some((id, name))
}
