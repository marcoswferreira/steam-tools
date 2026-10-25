pub mod cdp;
pub mod discovery;
pub mod grid;
pub mod vdf;

pub use cdp::{
    classify_target, format_cdp_call, trigger_steam_toast_cdp, AchievementToast, CdpTarget,
    TargetKind,
};
pub use discovery::{
    find_all_steam_shortcuts_candidates, find_steam_shortcuts_vdf, find_steam_userdata_dirs,
    is_steam_running, resolve_steam_shortcuts_path, safe_write_shortcuts_vdf,
};
pub use grid::SteamGridManager;
pub use vdf::{
    calculate_shortcut_id, encode_binary_vdf, parse_binary_vdf, upsert_shortcut, SteamShortcut,
    VdfNode, VdfValue,
};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_calculate_shortcut_id() {
        let exe = r#""C:\Emulators\ppsspp.exe""#;
        let title = "Crisis Core";
        let id = calculate_shortcut_id(exe, title);
        assert!(
            id >= 0x8000_0000,
            "Shortcut ID da Steam deve ter o bit 31 setado"
        );
    }

    #[test]
    fn test_vdf_roundtrip() {
        let original_nodes = vec![
            VdfNode {
                key: "AppName".to_string(),
                value: VdfValue::String("God of War".to_string()),
            },
            VdfNode {
                key: "appid".to_string(),
                value: VdfValue::Int32(12345),
            },
            VdfNode {
                key: "rating".to_string(),
                value: VdfValue::Float32(9.5),
            },
            VdfNode {
                key: "ptr".to_string(),
                value: VdfValue::Pointer(0xdeadbeef),
            },
            VdfNode {
                key: "playtime_64".to_string(),
                value: VdfValue::Int64(9876543210),
            },
        ];

        let bytes = encode_binary_vdf(&original_nodes).expect("Encoding VDF falhou");
        let parsed = parse_binary_vdf(&bytes).expect("Parsing VDF falhou");
        assert_eq!(original_nodes, parsed);
    }

    #[test]
    fn test_upsert_shortcut() {
        let mut root = Vec::new();
        let shortcut = SteamShortcut {
            appid: 0,
            app_name: "Persona 3 Portable".to_string(),
            exe: r#"C:\Emulators\PPSSPP.exe"#.to_string(),
            start_dir: r#"C:\Emulators"#.to_string(),
            launch_options: r#""C:\Games\p3p.iso""#.to_string(),
            icon: "".to_string(),
            tags: vec!["PSP".to_string(), "RPG".to_string()],
        };

        let generated_id = upsert_shortcut(&mut root, shortcut);
        assert!(generated_id >= 0x8000_0000);

        let shortcuts_node = root.iter().find(|n| n.key == "shortcuts").unwrap();
        match &shortcuts_node.value {
            VdfValue::Object(items) => {
                assert_eq!(items.len(), 1);
            }
            _ => panic!("shortcuts deve ser um objeto"),
        }
    }

    #[test]
    fn test_grid_paths() {
        let dir = tempdir().unwrap();
        let manager = SteamGridManager::new(dir.path().to_path_buf());
        let id = 3000000000;

        assert_eq!(manager.cover_path(id), dir.path().join("3000000000p.png"));
        assert_eq!(
            manager.hero_path(id),
            dir.path().join("3000000000_hero.png")
        );
        assert_eq!(
            manager.logo_path(id),
            dir.path().join("3000000000_logo.png")
        );
        assert_eq!(manager.wide_path(id), dir.path().join("3000000000.png"));
        assert_eq!(
            manager.icon_path(id),
            dir.path().join("3000000000_icon.ico")
        );

        let saved = manager.save_art(id, "cover", b"fake_png").unwrap();
        assert!(saved.exists());
        let saved_hero = manager.save_art(id, "hero", b"fake_hero").unwrap();
        assert!(saved_hero.exists());
        let saved_logo = manager.save_art(id, "logo", b"fake_logo").unwrap();
        assert!(saved_logo.exists());
        let saved_wide = manager.save_art(id, "wide", b"fake_wide").unwrap();
        assert!(saved_wide.exists());
        let saved_icon = manager.save_art(id, "icon", b"fake_icon").unwrap();
        assert!(saved_icon.exists());

        let src_file = dir.path().join("source_cover.png");
        std::fs::write(&src_file, b"sample bytes").unwrap();
        let copied = manager.copy_art_file(id, "cover", &src_file).unwrap();
        assert!(copied.exists());
        assert_eq!(std::fs::read(copied).unwrap(), b"sample bytes");
    }

    #[test]
    fn test_classify_cdp_target() {
        assert_eq!(
            classify_target("SharedJSContext", "https://steamloopback.host"),
            TargetKind::SharedJsContext
        );
        assert_eq!(
            classify_target("Steam Big Picture Mode", ""),
            TargetKind::BigPicture
        );
        assert_eq!(
            classify_target("Steam", "https://steamloopback.host/index.html"),
            TargetKind::Library
        );
        assert_eq!(
            classify_target("", "https://steamcommunity.com/profiles/123"),
            TargetKind::Community
        );
        assert_eq!(
            classify_target("DevTools", "about:blank"),
            TargetKind::Other
        );
    }

    #[test]
    fn test_achievement_toast_and_cdp() {
        let toast = AchievementToast {
            id: "trophy_1".to_string(),
            title: "Super Star".to_string(),
            description: "Got all stars".to_string(),
            icon_url: "https://example.com/icon.png".to_string(),
            app_id: 12345,
        };

        let script = toast.build_eval_script();
        assert!(script.contains("Super Star"));
        assert!(script.contains("Got all stars"));
        assert!(script.contains("12345"));

        let cdp_msg = format_cdp_call(
            1,
            "Runtime.evaluate",
            serde_json::json!({"expression": script}),
            Some("session123"),
        );
        assert!(cdp_msg.contains(r#""id":1"#));
        assert!(cdp_msg.contains(r#""method":"Runtime.evaluate""#));
        assert!(cdp_msg.contains(r#""sessionId":"session123""#));
    }
}
