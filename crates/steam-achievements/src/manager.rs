use crate::models::{Achievement, UnlockEvent};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub struct AchievementManager {
    pub storage_dir: PathBuf,
    pub cache: HashMap<String, Vec<Achievement>>,
    pub activity: Vec<UnlockEvent>,
}

impl AchievementManager {
    pub fn new(storage_dir: PathBuf) -> Self {
        let mut mgr = Self {
            storage_dir,
            cache: HashMap::new(),
            activity: Vec::new(),
        };
        mgr.load_activity();
        mgr
    }

    fn activity_file(&self) -> PathBuf {
        self.storage_dir.join("activity.json")
    }

    pub fn load_activity(&mut self) {
        let file = self.activity_file();
        if file.is_file() {
            if let Ok(bytes) = fs::read(&file) {
                if let Ok(events) = serde_json::from_slice::<Vec<UnlockEvent>>(&bytes) {
                    self.activity = events;
                }
            }
        }
    }

    pub fn save_activity(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.storage_dir)?;
        let data = serde_json::to_vec_pretty(&self.activity)?;
        fs::write(self.activity_file(), data)?;
        Ok(())
    }

    pub fn register_unlock(
        &mut self,
        game_id: &str,
        achievement_id: &str,
        title: &str,
        description: &str,
        icon: &str,
        source: &str,
    ) -> UnlockEvent {
        let event = UnlockEvent {
            id: Uuid::new_v4().to_string(),
            game_id: game_id.to_string(),
            achievement_id: achievement_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            icon: icon.to_string(),
            source: source.to_string(),
            unlocked_at: chrono::Utc::now().to_rfc3339(),
            steam_notified: false,
        };

        self.activity.push(event.clone());
        if self.activity.len() > 1000 {
            self.activity.drain(0..self.activity.len() - 1000);
        }
        let _ = self.save_activity();
        event
    }

    pub fn mark_notified(&mut self, event_id: &str) {
        if let Some(ev) = self.activity.iter_mut().find(|e| e.id == event_id) {
            ev.steam_notified = true;
            let _ = self.save_activity();
        }
    }
}
