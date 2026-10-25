pub mod disc;
pub mod emulators;
pub mod headers;
pub mod models;
pub mod pe;
pub mod platforms;
pub mod scanner;

pub use disc::{inspect_disc, DiscInfo};
pub use emulators::{
    detected_arguments, find_emulator_by_key, guess_profile_from_exe, standard_discovery_roots,
    EmulatorProfile, EMULATORS,
};
pub use headers::{read_cartridge_header, CartridgeHeader};
pub use models::{ArtSet, Game, LaunchBehavior, StoreInfo};
pub use pe::{detect_steam_app_id_in_dir, find_steam_app_id_near, SteamEvidence};
pub use platforms::{
    find_platform_by_key, guess_platform_by_ext, guess_platform_by_folder_name, PlatformDef,
    PLATFORMS,
};
pub use scanner::{scan_path_for_game, scan_roots_parallel};

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, ByteOrder, LittleEndian};
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_platform_lookup() {
        let psp = find_platform_by_key("psp").expect("PSP deve existir no catálogo");
        assert_eq!(psp.short, "PSP");
        assert_eq!(psp.igdb_id, Some(38));

        let ps2 = find_platform_by_key("ps2").expect("PS2 deve existir no catálogo");
        assert_eq!(ps2.short, "PS2");

        let ext_def = guess_platform_by_ext("cso").expect("CSO deve ser PSP");
        assert_eq!(ext_def.key, "psp");

        let folder_def =
            guess_platform_by_folder_name("switch").expect("Pasta switch deve ser reconhecida");
        assert_eq!(folder_def.key, "switch");
    }

    #[test]
    fn test_emulator_profiles() {
        let pcsx2 = find_emulator_by_key("ps2").expect("PCSX2 deve estar configurado");
        assert!(pcsx2.fullscreen_args.contains("-fullscreen"));

        let ppsspp = find_emulator_by_key("psp").expect("PPSSPP deve estar configurado");
        assert!(ppsspp.fullscreen_args.contains("--fullscreen"));

        let guessed =
            guess_profile_from_exe(Path::new("Dolphin.exe")).expect("Dolphin deve ser reconhecido");
        assert_eq!(guessed.key, "dolphin");

        let roots = standard_discovery_roots();
        assert!(!roots.is_empty());
    }

    #[test]
    fn test_gba_header_parsing() {
        let dir = tempdir().unwrap();
        let rom_path = dir.path().join("test_game.gba");
        let mut rom_bytes = vec![0u8; 0x200];
        rom_bytes[0x03] = 0xea;
        rom_bytes[0xb2] = 0x96;
        let title_bytes = b"POKEMON EMER";
        rom_bytes[0xa0..0xa0 + title_bytes.len()].copy_from_slice(title_bytes);
        rom_bytes[0xac..0xb0].copy_from_slice(b"BPEE");

        let mut file = std::fs::File::create(&rom_path).unwrap();
        file.write_all(&rom_bytes).unwrap();

        let header =
            read_cartridge_header(&rom_path, "gba").expect("Deve reconhecer cabeçalho GBA");
        assert_eq!(header.platform, "gba");
        assert_eq!(header.title, "POKEMON EMER");
        assert_eq!(header.serial, "BPEE");
    }

    #[test]
    fn test_nds_header_parsing() {
        let dir = tempdir().unwrap();
        let rom_path = dir.path().join("test_game.nds");
        let mut rom_bytes = vec![0u8; 0x200];
        rom_bytes[0..12].copy_from_slice(b"MARIO KART  ");
        rom_bytes[0x0c..0x10].copy_from_slice(b"AMKE");
        LittleEndian::write_u16(&mut rom_bytes[0x15c..0x15e], 0xcf56);

        std::fs::write(&rom_path, &rom_bytes).unwrap();
        let header = read_cartridge_header(&rom_path, "nds").expect("Deve ler NDS");
        assert_eq!(header.platform, "nds");
        assert_eq!(header.title, "MARIO KART");
        assert_eq!(header.serial, "AMKE");
    }

    #[test]
    fn test_gb_and_gbc_header_parsing() {
        let dir = tempdir().unwrap();
        let rom_path = dir.path().join("zelda.gb");
        let mut rom_bytes = vec![0u8; 0x200];
        BigEndian::write_u32(&mut rom_bytes[0x104..0x108], 0xceed6666);
        rom_bytes[0x134..0x139].copy_from_slice(b"ZELDA");
        rom_bytes[0x143] = 0x00;

        std::fs::write(&rom_path, &rom_bytes).unwrap();
        let header = read_cartridge_header(&rom_path, "gb").expect("Deve ler Game Boy");
        assert_eq!(header.platform, "gb");
        assert_eq!(header.title, "ZELDA");

        // GBC
        let gbc_path = dir.path().join("zelda.gbc");
        rom_bytes[0x143] = 0xc0;
        std::fs::write(&gbc_path, &rom_bytes).unwrap();
        let gbc_header = read_cartridge_header(&gbc_path, "gbc").expect("Deve ler GBC");
        assert_eq!(gbc_header.platform, "gbc");
    }

    #[test]
    fn test_n64_and_genesis_and_snes_headers() {
        let dir = tempdir().unwrap();

        // N64
        let n64_path = dir.path().join("sm64.z64");
        let mut n64_bytes = vec![0u8; 0x200];
        BigEndian::write_u32(&mut n64_bytes[0..4], 0x80371240);
        n64_bytes[0x20..0x20 + 11].copy_from_slice(b"SUPER MARIO");
        n64_bytes[0x3b..0x3f].copy_from_slice(b"NSME");
        std::fs::write(&n64_path, &n64_bytes).unwrap();
        let n64_head = read_cartridge_header(&n64_path, "z64").expect("N64");
        assert_eq!(n64_head.platform, "n64");
        assert_eq!(n64_head.serial, "NSME");

        // Genesis
        let md_path = dir.path().join("sonic.bin");
        let mut md_bytes = vec![0u8; 0x300];
        md_bytes[0x100..0x104].copy_from_slice(b"SEGA");
        md_bytes[0x150..0x150 + 11].copy_from_slice(b"SONIC HEDGE");
        md_bytes[0x180..0x184].copy_from_slice(b"1234");
        std::fs::write(&md_path, &md_bytes).unwrap();
        let md_head = read_cartridge_header(&md_path, "bin").expect("Genesis");
        assert_eq!(md_head.platform, "genesis");

        // SNES
        let snes_path = dir.path().join("smw.sfc");
        let mut snes_bytes = vec![0u8; 0x8000];
        LittleEndian::write_u16(&mut snes_bytes[0x7fc0 + 0x1c..0x7fc0 + 0x1e], 0x1234);
        LittleEndian::write_u16(
            &mut snes_bytes[0x7fc0 + 0x1e..0x7fc0 + 0x20],
            0x1234 ^ 0xffff,
        );
        snes_bytes[0x7fc0..0x7fc0 + 11].copy_from_slice(b"SUPER MARIO");
        std::fs::write(&snes_path, &snes_bytes).unwrap();
        let snes_head = read_cartridge_header(&snes_path, "sfc").expect("SNES");
        assert_eq!(snes_head.platform, "snes");
    }

    #[test]
    fn test_disc_inspection() {
        let dir = tempdir().unwrap();
        let iso_path = dir.path().join("game.iso");
        let mut iso_bytes = vec![0u8; 0x10000];
        // Setor 16 (0x8000) com CD001
        iso_bytes[0x8000..0x8006].copy_from_slice(b"\x01CD001");
        iso_bytes[0x8000 + 8..0x8000 + 19].copy_from_slice(b"PLAYSTATION");
        iso_bytes[0x8000 + 40..0x8000 + 51].copy_from_slice(b"MGS_DISC1  ");
        // Emulando BOOT2 em SYSTEM.CNF
        let cnf = b"BOOT2 = cdrom0:\\SLUS_200.62;1\n";
        iso_bytes[0x9000..0x9000 + cnf.len()].copy_from_slice(cnf);

        std::fs::write(&iso_path, &iso_bytes).unwrap();
        let disc = inspect_disc(&iso_path).expect("Deve inspecionar ISO de PS2");
        assert_eq!(disc.platform, "ps2");
        assert_eq!(disc.serial, Some("SLUS20062".to_string()));
    }

    #[test]
    fn test_steam_appid_detection() {
        let dir = tempdir().unwrap();
        let appid_file = dir.path().join("steam_appid.txt");
        std::fs::write(&appid_file, "400\n").unwrap(); // Portal

        let ev = detect_steam_app_id_in_dir(dir.path()).expect("Deve ler steam_appid.txt");
        assert_eq!(ev.app_id, 400);

        // Subdiretório e find_steam_app_id_near
        let sub = dir.path().join("bin");
        std::fs::create_dir_all(&sub).unwrap();
        let exe = sub.join("game.exe");
        std::fs::write(&exe, b"").unwrap();
        let near = find_steam_app_id_near(&exe).expect("Deve achar appid no diretório pai");
        assert_eq!(near.app_id, 400);
    }

    #[test]
    fn test_scanner_parallel() {
        let dir = tempdir().unwrap();
        let psp_dir = dir.path().join("psp");
        std::fs::create_dir_all(&psp_dir).unwrap();
        let rom = psp_dir.join("game.cso");
        std::fs::write(&rom, b"dummy cso").unwrap();

        let scanned = scan_roots_parallel(&[dir.path().to_path_buf()], 3);
        assert!(!scanned.is_empty());
        assert_eq!(scanned[0].platform, "psp");
    }
}
