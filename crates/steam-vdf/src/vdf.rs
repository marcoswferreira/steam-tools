use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc32fast::Hasher;
use std::io::{Cursor, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum VdfValue {
    Object(Vec<VdfNode>),
    String(String),
    Int32(i32),
    Float32(f32),
    Pointer(u32),
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VdfNode {
    pub key: String,
    pub value: VdfValue,
}

pub fn calculate_shortcut_id(exe: &str, app_name: &str) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(exe.as_bytes());
    hasher.update(app_name.as_bytes());
    let crc = hasher.finalize();
    crc | 0x8000_0000
}

pub fn read_cstring<R: Read>(reader: &mut R) -> std::io::Result<String> {
    let mut bytes = Vec::new();
    loop {
        let b = reader.read_u8()?;
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

pub fn write_cstring<W: Write>(writer: &mut W, s: &str) -> std::io::Result<()> {
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

pub fn parse_binary_vdf(bytes: &[u8]) -> std::io::Result<Vec<VdfNode>> {
    let mut cursor = Cursor::new(bytes);
    parse_nodes(&mut cursor, 0)
}

fn parse_nodes<R: Read>(reader: &mut R, depth: usize) -> std::io::Result<Vec<VdfNode>> {
    if depth > 32 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "VDF recursão excessiva",
        ));
    }
    let mut nodes = Vec::new();
    loop {
        let node_type = match reader.read_u8() {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        if node_type == 0x08 {
            break;
        }

        let key = read_cstring(reader)?;
        let value = match node_type {
            0x00 => VdfValue::Object(parse_nodes(reader, depth + 1)?),
            0x01 => VdfValue::String(read_cstring(reader)?),
            0x02 => VdfValue::Int32(reader.read_i32::<LittleEndian>()?),
            0x03 => VdfValue::Float32(reader.read_f32::<LittleEndian>()?),
            0x04 => VdfValue::Pointer(reader.read_u32::<LittleEndian>()?),
            0x07 => VdfValue::Int64(reader.read_i64::<LittleEndian>()?),
            other => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Tipo de nó VDF não suportado: 0x{:02x}", other),
                ));
            }
        };

        nodes.push(VdfNode { key, value });
    }
    Ok(nodes)
}

pub fn encode_binary_vdf(nodes: &[VdfNode]) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    encode_nodes(&mut buffer, nodes)?;
    Ok(buffer)
}

fn encode_nodes<W: Write>(writer: &mut W, nodes: &[VdfNode]) -> std::io::Result<()> {
    for node in nodes {
        match &node.value {
            VdfValue::Object(children) => {
                writer.write_u8(0x00)?;
                write_cstring(writer, &node.key)?;
                encode_nodes(writer, children)?;
            }
            VdfValue::String(s) => {
                writer.write_u8(0x01)?;
                write_cstring(writer, &node.key)?;
                write_cstring(writer, s)?;
            }
            VdfValue::Int32(n) => {
                writer.write_u8(0x02)?;
                write_cstring(writer, &node.key)?;
                writer.write_i32::<LittleEndian>(*n)?;
            }
            VdfValue::Float32(f) => {
                writer.write_u8(0x03)?;
                write_cstring(writer, &node.key)?;
                writer.write_f32::<LittleEndian>(*f)?;
            }
            VdfValue::Pointer(p) => {
                writer.write_u8(0x04)?;
                write_cstring(writer, &node.key)?;
                writer.write_u32::<LittleEndian>(*p)?;
            }
            VdfValue::Int64(n) => {
                writer.write_u8(0x07)?;
                write_cstring(writer, &node.key)?;
                writer.write_i64::<LittleEndian>(*n)?;
            }
        }
    }
    writer.write_u8(0x08)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct SteamShortcut {
    pub appid: u32,
    pub app_name: String,
    pub exe: String,
    pub start_dir: String,
    pub launch_options: String,
    pub icon: String,
    pub tags: Vec<String>,
}

pub fn upsert_shortcut(root_nodes: &mut Vec<VdfNode>, shortcut: SteamShortcut) -> u32 {
    let shortcuts_idx = match root_nodes
        .iter()
        .position(|n| n.key.eq_ignore_ascii_case("shortcuts"))
    {
        Some(idx) => idx,
        None => {
            root_nodes.push(VdfNode {
                key: "shortcuts".to_string(),
                value: VdfValue::Object(Vec::new()),
            });
            root_nodes.len() - 1
        }
    };

    let list = match &mut root_nodes[shortcuts_idx].value {
        VdfValue::Object(l) => l,
        _ => return shortcut.appid,
    };

    let target_app_id = if shortcut.appid > 0 {
        shortcut.appid
    } else {
        calculate_shortcut_id(&shortcut.exe, &shortcut.app_name)
    };

    let mut entry_nodes = Vec::new();
    entry_nodes.push(VdfNode {
        key: "appid".to_string(),
        value: VdfValue::Int32(target_app_id as i32),
    });
    entry_nodes.push(VdfNode {
        key: "AppName".to_string(),
        value: VdfValue::String(shortcut.app_name),
    });
    entry_nodes.push(VdfNode {
        key: "exe".to_string(),
        value: VdfValue::String(format!(r#""{}""#, shortcut.exe.trim_matches('"'))),
    });
    entry_nodes.push(VdfNode {
        key: "StartDir".to_string(),
        value: VdfValue::String(format!(r#""{}""#, shortcut.start_dir.trim_matches('"'))),
    });
    entry_nodes.push(VdfNode {
        key: "LaunchOptions".to_string(),
        value: VdfValue::String(shortcut.launch_options),
    });
    entry_nodes.push(VdfNode {
        key: "icon".to_string(),
        value: VdfValue::String(shortcut.icon),
    });
    entry_nodes.push(VdfNode {
        key: "IsHidden".to_string(),
        value: VdfValue::Int32(0),
    });
    entry_nodes.push(VdfNode {
        key: "AllowDesktopConfig".to_string(),
        value: VdfValue::Int32(1),
    });
    entry_nodes.push(VdfNode {
        key: "AllowOverlay".to_string(),
        value: VdfValue::Int32(1),
    });
    entry_nodes.push(VdfNode {
        key: "OpenVR".to_string(),
        value: VdfValue::Int32(0),
    });
    entry_nodes.push(VdfNode {
        key: "Devkit".to_string(),
        value: VdfValue::Int32(0),
    });
    entry_nodes.push(VdfNode {
        key: "DevkitGameID".to_string(),
        value: VdfValue::String("".to_string()),
    });
    entry_nodes.push(VdfNode {
        key: "DevkitOverrideAppID".to_string(),
        value: VdfValue::Int32(0),
    });
    entry_nodes.push(VdfNode {
        key: "LastPlayTime".to_string(),
        value: VdfValue::Int32(0),
    });
    entry_nodes.push(VdfNode {
        key: "FlatpakAppID".to_string(),
        value: VdfValue::String("".to_string()),
    });

    let tag_nodes: Vec<VdfNode> = shortcut
        .tags
        .into_iter()
        .enumerate()
        .map(|(i, tag)| VdfNode {
            key: i.to_string(),
            value: VdfValue::String(tag),
        })
        .collect();
    entry_nodes.push(VdfNode {
        key: "tags".to_string(),
        value: VdfValue::Object(tag_nodes),
    });

    let existing_pos = list.iter().position(|item| match &item.value {
        VdfValue::Object(fields) => fields.iter().any(|f| {
            f.key.eq_ignore_ascii_case("appid")
                && match f.value {
                    VdfValue::Int32(id) => (id as u32) == target_app_id,
                    _ => false,
                }
        }),
        _ => false,
    });

    if let Some(pos) = existing_pos {
        list[pos] = VdfNode {
            key: pos.to_string(),
            value: VdfValue::Object(entry_nodes),
        };
    } else {
        let new_key = list.len().to_string();
        list.push(VdfNode {
            key: new_key,
            value: VdfValue::Object(entry_nodes),
        });
    }

    target_app_id
}
