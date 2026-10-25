use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartridgeHeader {
    pub platform: &'static str,
    pub title: String,
    pub serial: String,
}

fn clean_ascii(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        if b == 0 {
            break;
        }
        if (0x20..=0x7e).contains(&b) {
            s.push(b as char);
        } else {
            s.push(' ');
        }
    }
    s.trim().to_string()
}

#[allow(clippy::chunks_exact_to_as_chunks)]
pub fn read_cartridge_header(path: &Path, ext: &str) -> Option<CartridgeHeader> {
    let mut file = File::open(path).ok()?;
    let mut buf = [0u8; 0x10200];
    let len = file.read(&mut buf).ok()?;
    if len < 0x200 {
        return None;
    }
    let data = &buf[..len];
    let ext_lower = ext.trim_start_matches('.').to_ascii_lowercase();

    // 1. GBA (Game Boy Advance): logo Nintendo em 0x04..0xa0 e byte 0x96 em 0xb2
    if len >= 0xc0 && data[0xb2] == 0x96 && data[0x03] == 0xea {
        let title = clean_ascii(&data[0xa0..0xac]);
        let serial = clean_ascii(&data[0xac..0xb0]);
        if serial.len() == 4 && serial.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Some(CartridgeHeader {
                platform: "gba",
                title,
                serial,
            });
        }
    }

    // 2. NDS (Nintendo DS): código do jogo em 0x0c..0x10 e CRC do logo (0xcf56) em 0x15c
    if len >= 0x160 {
        let logo_crc = LittleEndian::read_u16(&data[0x15c..0x15e]);
        if logo_crc == 0xcf56 {
            let title = clean_ascii(&data[0x00..0x0c]);
            let serial = clean_ascii(&data[0x0c..0x10]);
            return Some(CartridgeHeader {
                platform: "nds",
                title,
                serial,
            });
        }
    }

    // 3. Game Boy / Game Boy Color: Logo em 0x104 (CE ED 66 66)
    if len >= 0x150 && BigEndian::read_u32(&data[0x104..0x108]) == 0xceed6666 {
        let cgb = data[0x143] == 0x80 || data[0x143] == 0xc0;
        let title = clean_ascii(&data[0x134..if cgb { 0x13f } else { 0x144 }]);
        let platform = if data[0x143] == 0xc0 || (cgb && ext_lower == "gbc") {
            "gbc"
        } else {
            "gb"
        };
        return Some(CartridgeHeader {
            platform,
            title,
            serial: String::new(),
        });
    }

    // 4. Nintendo 64: Magic 0x80371240 (z64), 0x37804012 (v64 byteswapped), 0x40123780 (n64 word-swapped)
    if len >= 0x40 {
        let magic = BigEndian::read_u32(&data[0x00..0x04]);
        if magic == 0x80371240 || magic == 0x37804012 || magic == 0x40123780 {
            let mut header = [0u8; 0x40];
            header.copy_from_slice(&data[0x00..0x40]);
            if magic == 0x37804012 {
                for chunk in header.chunks_exact_mut(2) {
                    chunk.swap(0, 1);
                }
            } else if magic == 0x40123780 {
                for chunk in header.chunks_exact_mut(4) {
                    chunk.reverse();
                }
            }
            let title = clean_ascii(&header[0x20..0x34]);
            let serial = clean_ascii(&header[0x3b..0x3f]);
            return Some(CartridgeHeader {
                platform: "n64",
                title,
                serial,
            });
        }
    }

    // 5. Mega Drive / Genesis: "SEGA" em 0x100
    if len >= 0x200 && &data[0x100..0x104] == b"SEGA" {
        let sys = clean_ascii(&data[0x100..0x110]);
        let domestic = clean_ascii(&data[0x120..0x150]);
        let overseas = clean_ascii(&data[0x150..0x180]);
        let serial = clean_ascii(&data[0x180..0x18e]);
        let title = if !overseas.is_empty() {
            overseas
        } else {
            domestic
        };
        let platform = if sys.contains("32X") {
            "sega32x"
        } else {
            "genesis"
        };
        return Some(CartridgeHeader {
            platform,
            title,
            serial,
        });
    }

    // 6. SNES: LoROM (0x7FC0) ou HiROM (0xFFC0), verificação de checksum ^ complement == 0xFFFF
    if ["sfc", "smc", "swc", "fig"].contains(&ext_lower.as_str()) {
        for &base in &[0x7fc0usize, 0xffc0usize, 0x81c0usize, 0x101c0usize] {
            if len >= base + 0x30 {
                let complement = LittleEndian::read_u16(&data[base + 0x1c..base + 0x1e]);
                let checksum = LittleEndian::read_u16(&data[base + 0x1e..base + 0x20]);
                if (checksum ^ complement) == 0xffff {
                    let title = clean_ascii(&data[base..base + 21]);
                    return Some(CartridgeHeader {
                        platform: "snes",
                        title,
                        serial: String::new(),
                    });
                }
            }
        }
    }

    // 7. NES: iNES Magic b"NES\x1a"
    if len >= 16 && &data[0..4] == b"NES\x1a" {
        return Some(CartridgeHeader {
            platform: "nes",
            title: String::new(),
            serial: String::new(),
        });
    }

    None
}
