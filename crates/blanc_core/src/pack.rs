use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const PACK_MAGIC: &[u8; 8] = b"BLNCPAK1";
pub const EXPORT_META_ENTRY: &str = "__blanc_export_meta.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ExportSaveMode {
    #[default]
    Portable,
    AppData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExportMetadata {
    pub save_mode: ExportSaveMode,
    pub game_title: String,
}

impl Default for ExportMetadata {
    fn default() -> Self {
        Self {
            save_mode: ExportSaveMode::Portable,
            game_title: "BlancGame".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackEntry {
    path: String,
    offset: u64,
    size: u64,
    nonce: u64,
}

/// Deterministic 64-byte block mixer for stream encryption (per-export 256-bit seed + nonce).
fn keystream_block(seed: &[u8; 32], nonce: u64, counter: u64) -> [u8; 64] {
    let mut s = [0u64; 8];
    for i in 0..4 {
        let base = i * 8;
        s[i] = u64::from_le_bytes([
            seed[base],
            seed[base + 1],
            seed[base + 2],
            seed[base + 3],
            seed[base + 4],
            seed[base + 5],
            seed[base + 6],
            seed[base + 7],
        ]);
    }
    s[4] = nonce ^ 0x9E37_79B9_7F4A_7C15;
    s[5] = counter ^ 0xBF58_476D_1CE4_E5B9;
    s[6] = s[0].wrapping_add(s[2]) ^ 0x94D0_49BB_1331_11EB;
    s[7] = s[1].wrapping_add(s[3]) ^ 0xD6E8_FEB8_6659_FD93;

    for _ in 0..8 {
        for i in 0..8 {
            let next = (i + 1) % 8;
            let other = (i + 3) % 8;
            s[i] = s[i].wrapping_add(s[other]).rotate_left(17) ^ s[next];
            s[next] = s[next].wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_right(11);
        }
    }

    let mut out = [0u8; 64];
    for i in 0..8 {
        out[i * 8..(i + 1) * 8].copy_from_slice(&s[i].to_le_bytes());
    }
    out
}

fn xor_keystream(seed: &[u8; 32], nonce: u64, data: &mut [u8]) {
    let mut counter = 0u64;
    for chunk in data.chunks_mut(64) {
        let block = keystream_block(seed, nonce, counter);
        for (b, k) in chunk.iter_mut().zip(block.iter()) {
            *b ^= *k;
        }
        counter += 1;
    }
}

fn normalize_pack_path(raw: &str) -> String {
    raw.replace('\\', "/").trim_start_matches("./").to_string()
}

/// Write an encrypted `.bpack` archive from `(relative_path, raw_bytes)` pairs.
pub fn write_bpack(output_path: &Path, files: &[(String, Vec<u8>)]) -> Result<(), String> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create output dir: {}", e))?;
    }

    let mut seed = [0u8; 32];
    seed[0..16].copy_from_slice(Uuid::new_v4().as_bytes());
    seed[16..32].copy_from_slice(Uuid::new_v4().as_bytes());

    let mut entries = Vec::with_capacity(files.len());
    let mut encrypted_blobs = Vec::with_capacity(files.len());
    let mut current_offset = 0u64;

    for (idx, (rel_path, bytes)) in files.iter().enumerate() {
        let norm = normalize_pack_path(rel_path);
        let nonce = (idx as u64).wrapping_add(1).wrapping_mul(0xA076_1D64_78BD_642F);
        let mut enc = bytes.clone();
        xor_keystream(&seed, nonce, &mut enc);
        let size = enc.len() as u64;
        entries.push(PackEntry {
            path: norm,
            offset: current_offset,
            size,
            nonce,
        });
        current_offset += size;
        encrypted_blobs.push(enc);
    }

    let mut index_bytes =
        serde_json::to_vec(&entries).map_err(|e| format!("Failed to serialize pack index: {}", e))?;
    xor_keystream(&seed, 0, &mut index_bytes);

    let mut out =
        File::create(output_path).map_err(|e| format!("Failed to create {}: {}", output_path.display(), e))?;
    out.write_all(PACK_MAGIC).map_err(|e| e.to_string())?;
    out.write_all(&seed).map_err(|e| e.to_string())?;
    out.write_all(&(entries.len() as u32).to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(&(index_bytes.len() as u32).to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(&index_bytes).map_err(|e| e.to_string())?;

    for blob in encrypted_blobs {
        out.write_all(&blob).map_err(|e| e.to_string())?;
    }
    out.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// Reader for an encrypted `.bpack` archive (keeps only the index in memory).
#[derive(Debug, Clone)]
pub struct BpackArchive {
    path: PathBuf,
    seed: [u8; 32],
    data_start: u64,
    entries: HashMap<String, PackEntry>,
}

impl BpackArchive {
    pub fn open(path: &Path) -> Result<Self, String> {
        let mut f = File::open(path).map_err(|e| format!("Cannot open {}: {}", path.display(), e))?;
        let mut magic = [0u8; 8];
        f.read_exact(&mut magic).map_err(|e| e.to_string())?;
        if &magic != PACK_MAGIC {
            return Err("Invalid bpack header".to_string());
        }

        let mut seed = [0u8; 32];
        f.read_exact(&mut seed).map_err(|e| e.to_string())?;

        let mut u32_buf = [0u8; 4];
        f.read_exact(&mut u32_buf).map_err(|e| e.to_string())?;
        let _entry_count = u32::from_le_bytes(u32_buf);

        f.read_exact(&mut u32_buf).map_err(|e| e.to_string())?;
        let index_len = u32::from_le_bytes(u32_buf) as usize;

        let mut index_bytes = vec![0u8; index_len];
        f.read_exact(&mut index_bytes).map_err(|e| e.to_string())?;
        xor_keystream(&seed, 0, &mut index_bytes);

        let parsed: Vec<PackEntry> =
            serde_json::from_slice(&index_bytes).map_err(|e| format!("Corrupt bpack index: {}", e))?;
        let data_start = 8 + 32 + 4 + 4 + index_len as u64;

        let mut entries = HashMap::with_capacity(parsed.len());
        for entry in parsed {
            entries.insert(entry.path.to_lowercase(), entry);
        }

        Ok(Self {
            path: path.to_path_buf(),
            seed,
            data_start,
            entries,
        })
    }

    pub fn read_file(&self, rel_path: &str) -> Option<Vec<u8>> {
        let key = normalize_pack_path(rel_path).to_lowercase();
        let entry = self.entries.get(&key)?;
        let mut f = File::open(&self.path).ok()?;
        f.seek(SeekFrom::Start(self.data_start + entry.offset)).ok()?;
        let mut buf = vec![0u8; entry.size as usize];
        f.read_exact(&mut buf).ok()?;
        xor_keystream(&self.seed, entry.nonce, &mut buf);
        Some(buf)
    }

    pub fn list_files(&self) -> Vec<&str> {
        self.entries.values().map(|e| e.path.as_str()).collect()
    }
}
