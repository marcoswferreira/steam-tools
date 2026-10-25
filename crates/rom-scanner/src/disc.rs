use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscInfo {
    pub platform: &'static str,
    pub title: Option<String>,
    pub serial: Option<String>,
    pub volume_id: String,
}

pub fn inspect_disc(path: &Path) -> Option<DiscInfo> {
    let mut file = File::open(path).ok()?;
    let meta = file.metadata().ok()?;
    let size = meta.len();
    if size < 0x9000 {
        return None;
    }

    // Leitura do Primary Volume Descriptor (PVD) da ISO9660 em LBA 16 (0x8000)
    let mut sector = [0u8; 2048];
    file.seek(SeekFrom::Start(0x8000)).ok()?;
    if file.read_exact(&mut sector).is_err() {
        return None;
    }

    // Verificar assinatura b"\x01CD001\x01"
    if &sector[0..6] != b"\x01CD001" {
        // Tentar detecção crua em BIN/CUE (setor de 2352 bytes com cabeçalho sync de 16 bytes)
        file.seek(SeekFrom::Start(16 * 2352 + 16)).ok()?;
        if file.read_exact(&mut sector).is_err() || &sector[0..6] != b"\x01CD001" {
            return None;
        }
    }

    let sys_id = String::from_utf8_lossy(&sector[8..40]).trim().to_string();
    let vol_id = String::from_utf8_lossy(&sector[40..72]).trim().to_string();

    let mut found_serial = None;
    let mut platform = "pc";

    if sys_id.contains("PLAYSTATION") {
        platform = "ps1";
    }

    // Tentar localizar strings de SYSTEM.CNF ou serials comuns de PlayStation nos primeiros 2MB da imagem
    let scan_limit = size.min(2 * 1024 * 1024) as usize;
    let mut scan_buf = vec![0u8; scan_limit];
    file.seek(SeekFrom::Start(0)).ok()?;
    if let Ok(bytes_read) = file.read(&mut scan_buf) {
        let content = String::from_utf8_lossy(&scan_buf[..bytes_read]);

        // Busca padrão: BOOT2 = cdrom0:\SLUS_200.62;1 ou BOOT = cdrom0:\SLES_123.45;1
        let re_system_cnf =
            regex::Regex::new(r#"(?i)BOOT\d?\s*=\s*cdrom0?:\\?([A-Z]{4}[-_ ]?\d{3}\.?\d{2})"#)
                .ok()?;
        if let Some(caps) = re_system_cnf.captures(&content) {
            let s = caps.get(1)?.as_str().replace(['-', '_', '.'], "");
            found_serial = Some(s);
            platform = if content.contains("BOOT2") {
                "ps2"
            } else {
                "ps1"
            };
        } else {
            // Busca por serials de PSP / PS3 / PS4 em PARAM.SFO
            let re_ps =
                regex::Regex::new(r#"(?i)\b((?:UL|UC|NP|ND|BL|BC)[A-Z]{2}\d{5}|CUSA\d{5})\b"#)
                    .ok()?;
            if let Some(caps) = re_ps.captures(&content) {
                let s = caps.get(1)?.as_str().to_string();
                found_serial = Some(s.clone());
                if s.starts_with("UL") || s.starts_with("UC") || s.starts_with("NP") {
                    platform = "psp";
                } else if s.starts_with("BL") || s.starts_with("BC") {
                    platform = "ps3";
                } else if s.starts_with("CUSA") {
                    platform = "ps4";
                }
            }
        }
    }

    Some(DiscInfo {
        platform,
        title: if !vol_id.is_empty() {
            Some(vol_id.clone())
        } else {
            None
        },
        serial: found_serial,
        volume_id: vol_id,
    })
}
