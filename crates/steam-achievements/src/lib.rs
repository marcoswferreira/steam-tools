pub mod manager;
pub mod models;

pub use manager::AchievementManager;
pub use models::{parse_rpcs3_trophy_log_line, Achievement, Rpcs3Trophy, UnlockEvent};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_rpcs3_log_parsing() {
        let line = "Trophy unlocked: [NPWR01234_00] Trophy ID 5 (Master of Arms)";
        let result = parse_rpcs3_trophy_log_line(line);
        assert_eq!(result, Some((5, "Master of Arms".to_string())));

        let bad_line = "Just another log message";
        assert_eq!(parse_rpcs3_trophy_log_line(bad_line), None);
    }

    #[test]
    fn test_register_and_notify_unlock() {
        let dir = tempdir().unwrap();
        let mut mgr = AchievementManager::new(dir.path().to_path_buf());

        let ev = mgr.register_unlock(
            "game_123",
            "ach_01",
            "First Step",
            "Completed the tutorial",
            "/icons/tut.png",
            "RetroAchievements",
        );

        assert_eq!(mgr.activity.len(), 1);
        assert_eq!(mgr.activity[0].title, "First Step");
        assert!(!mgr.activity[0].steam_notified);

        mgr.mark_notified(&ev.id);
        assert!(mgr.activity[0].steam_notified);

        // Validar recarregamento do arquivo
        let mgr2 = AchievementManager::new(dir.path().to_path_buf());
        assert_eq!(mgr2.activity.len(), 1);
        assert!(mgr2.activity[0].steam_notified);
    }
}
