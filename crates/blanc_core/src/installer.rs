use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub const INSTALLER_MAGIC: &[u8; 9] = b"BLNCINST1";
pub const UNINSTALL_META_FILENAME: &str = ".blanc_uninstall.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum InstallerLang {
    #[default]
    En,
    Id,
    Ja,
    Es,
    Zh,
    Ko,
}

impl InstallerLang {
    pub const ALL: [InstallerLang; 6] = [
        InstallerLang::En,
        InstallerLang::Id,
        InstallerLang::Ja,
        InstallerLang::Es,
        InstallerLang::Zh,
        InstallerLang::Ko,
    ];

    pub fn code(self) -> &'static str {
        match self {
            InstallerLang::En => "en",
            InstallerLang::Id => "id",
            InstallerLang::Ja => "ja",
            InstallerLang::Es => "es",
            InstallerLang::Zh => "zh",
            InstallerLang::Ko => "ko",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            InstallerLang::En => "EN",
            InstallerLang::Id => "ID",
            InstallerLang::Ja => "JA",
            InstallerLang::Es => "ES",
            InstallerLang::Zh => "ZH",
            InstallerLang::Ko => "KO",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            InstallerLang::En => "English",
            InstallerLang::Id => "Bahasa Indonesia",
            InstallerLang::Ja => "日本語 (Japanese)",
            InstallerLang::Es => "Español (Spanish)",
            InstallerLang::Zh => "简体中文 (Simplified Chinese)",
            InstallerLang::Ko => "한국어 (Korean)",
        }
    }
}

impl std::fmt::Display for InstallerLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

pub fn default_privacy_and_terms(lang: InstallerLang, game_title: &str, publisher: &str) -> String {
    match lang {
        InstallerLang::En => format!(
            "TERMS OF USE & PRIVACY POLICY\n\n\
1. Privacy & No Data Collection\n\
{game_title} is an offline desktop visual novel. This software does not collect, track, store, or transmit any personal information, usage analytics, or telemetry to any external servers or third parties.\n\n\
2. Local Save Data & Settings\n\
Game progress (save files) and user preferences (audio volume, text speed, display settings) are stored locally on your device. You may remove this data at any time during uninstallation.\n\n\
3. Intellectual Property & License\n\
All story content, characters, artwork, audio, and software assets in {game_title} are the property of {publisher}. You are granted a personal, non-exclusive, non-transferable license to install and play this game for personal, non-commercial use. Redistribution, resale, or unauthorized modification is prohibited.\n\n\
4. Disclaimer\n\
This software is provided \"AS IS\", without warranty of any kind, express or implied. {publisher} shall not be liable for any damages arising from the use of this software."
        ),
        InstallerLang::Id => format!(
            "KETENTUAN PENGGUNAAN & KEBIJAKAN PRIVASI\n\n\
1. Privasi & Tanpa Pengumpulan Data\n\
{game_title} adalah visual novel desktop offline. Perangkat lunak ini tidak mengumpulkan, melacak, menyimpan, atau mengirimkan informasi pribadi, analitik penggunaan, maupun telemetri ke server eksternal atau pihak ketiga mana pun.\n\n\
2. Data Save & Pengaturan Lokal\n\
Progres permainan (file save) dan preferensi pengguna (volume audio, kecepatan teks, pengaturan tampilan) disimpan secara lokal di perangkat Anda. Data ini dapat dihapus kapan saja saat proses uninstall.\n\n\
3. Hak Kekayaan Intelektual & Lisensi\n\
Seluruh cerita, karakter, ilustrasi, audio, dan aset perangkat lunak dalam {game_title} adalah milik {publisher}. Anda diberikan lisensi pribadi, non-eksklusif, dan tidak dapat dipindahtangankan untuk menginstall dan memainkan game ini untuk penggunaan pribadi non-komersial. Dilarang mendistribusikan ulang, menjual kembali, atau memodifikasi tanpa izin.\n\n\
4. Penafian (Disclaimer)\n\
Perangkat lunak ini disediakan \"SEBAGAIMANA ADANYA\" tanpa jaminan apa pun, baik tersurat maupun tersirat. {publisher} tidak bertanggung jawab atas kerugian apa pun yang timbul dari penggunaan perangkat lunak ini."
        ),
        InstallerLang::Ja => format!(
            "利用規約およびプライバシーポリシー\n\n\
1. プライバシーとデータ非収集\n\
{game_title} はオフラインのデスクトップ向けビジュアルノベルです。本ソフトウェアは、個人情報、利用状況の分析データ、テレメトリなどを外部サーバーや第三者に収集・追跡・保存・送信することは一切ありません。\n\n\
2. ローカルセーブデータと設定\n\
ゲームの進行状況（セーブデータ）およびユーザー設定（音量、テキスト速度、表示設定）は、お使いのデバイス上にローカル保存されます。これらのデータはアンインストール時にいつでも削除できます。\n\n\
3. 知的財産権とライセンス\n\
{game_title} に含まれるすべてのストーリー、キャラクター、イラスト、音声、およびソフトウェア資産の権利は {publisher} に帰属します。個人かつ非営利目的で本ゲームをインストールおよびプレイするための非独占的・譲渡不可のライセンスが付与されます。無断での再配布、転売、改変は禁止されています。\n\n\
4. 免責事項\n\
本ソフトウェアは「現状のまま」提供され、明示的または黙示的を問わずいかなる保証も行いません。本ソフトウェアの使用により生じたいかなる損害についても、{publisher} は責任を負いません。"
        ),
        InstallerLang::Es => format!(
            "TÉRMINOS DE USO Y POLÍTICA DE PRIVACIDAD\n\n\
1. Privacidad y cero recopilación de datos\n\
{game_title} es una novela visual de escritorio sin conexión. Este software no recopila, rastrea, almacena ni transmite información personal, analíticas de uso ni telemetría a servidores externos o terceros.\n\n\
2. Datos guardados y configuración local\n\
El progreso del juego (archivos de guardado) y las preferencias del usuario (volumen de audio, velocidad del texto, ajustes de pantalla) se almacenan localmente en tu dispositivo. Puedes eliminar estos datos en cualquier momento durante la desinstalación.\n\n\
3. Propiedad intelectual y licencia\n\
Todo el contenido de la historia, personajes, ilustraciones, audio y recursos de {game_title} son propiedad de {publisher}. Se te concede una licencia personal, no exclusiva e intransferible para instalar y jugar este título para uso personal y no comercial. Se prohíbe su redistribución, reventa o modificación no autorizada.\n\n\
4. Exención de responsabilidad\n\
Este software se proporciona \"TAL CUAL\", sin garantías de ningún tipo, expresas o implícitas. {publisher} no será responsable de ningún daño derivado del uso de este software."
        ),
        InstallerLang::Zh => format!(
            "使用条款与隐私政策\n\n\
1. 隐私保护与无数据收集\n\
{game_title} 是一款离线桌面视觉小说游戏。本软件不会收集、追踪、存储或向任何外部服务器或第三方传输您的个人信息、使用分析或遥测数据。\n\n\
2. 本地存档与设置\n\
游戏进度（存档文件）和用户偏好设置（音量、文本速度、显示设置）仅保存在您的本地设备中。您可以在卸载时随时选择删除这些数据。\n\n\
3. 知识产权与许可\n\
{game_title} 中的所有剧情内容、角色、美术、音频及软件资源均归 {publisher} 所有。您仅获得个人的、非排他性的、不可转让的许可，用于个人非商业用途安装和游玩本游戏。禁止未经授权的再分发、转售或修改。\n\n\
4. 免责声明\n\
本软件按“原样”提供，不提供任何明示或暗示的担保。对于因使用本软件而产生的任何损失，{publisher} 概不负责。"
        ),
        InstallerLang::Ko => format!(
            "이용약관 및 개인정보 처리방침\n\n\
1. 개인정보 보호 및 데이터 미수집\n\
{game_title}은(는) 오프라인 데스크톱 비주얼 노벨입니다. 본 소프트웨어는 개인정보, 사용 통계 또는 원격 측정 데이터를 외부 서버나 제3자에게 수집, 추적, 저장 또는 전송하지 않습니다.\n\n\
2. 로컬 저장 데이터 및 설정\n\
게임 진행 상황(저장 파일)과 사용자 설정(오디오 볼륨, 텍스트 속도, 화면 설정)은 사용자의 기기에 로컬로 저장됩니다. 이 데이터는 제거(언인스톨) 시 언제든지 삭제할 수 있습니다.\n\n\
3. 지식재산권 및 라이선스\n\
{game_title}의 모든 스토리, 캐릭터, 아트워크, 오디오 및 소프트웨어 자산의 권리는 {publisher}에 있습니다. 개인적·비상업적 용도로 본 게임을 설치하고 플레이할 수 있는 개인적이고 비독점적이며 양도 불가능한 라이선스가 부여됩니다. 무단 재배포, 재판매 또는 수정은 금지됩니다.\n\n\
4. 면책 조항\n\
본 소프트웨어는 어떠한 명시적 또는 묵시적 보증 없이 \"있는 그대로\" 제공됩니다. {publisher}은(는) 본 소프트웨어의 사용으로 인해 발생하는 어떠한 손해에 대해서도 책임을 지지 않습니다."
        ),
    }
}

pub fn default_privacy_and_terms_en(game_title: &str, publisher: &str) -> String {
    default_privacy_and_terms(InstallerLang::En, game_title, publisher)
}

pub fn default_privacy_and_terms_map() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for &lang in &InstallerLang::ALL {
        map.insert(
            lang.code().to_string(),
            default_privacy_and_terms(lang, "{GAME_TITLE}", "{PUBLISHER}"),
        );
    }
    map
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UninstallMetadata {
    pub game_title: String,
    pub version: String,
    pub publisher: String,
    pub game_exe_name: String,
    pub accent_hex: String,
    #[serde(default)]
    pub logo_png: Option<Vec<u8>>,
    #[serde(default)]
    pub selected_lang: Option<String>,
    #[serde(default)]
    pub installed_files: Vec<String>,
}

pub fn sanitize_installer_name(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == ' ' || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "BlancGame".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn uninstall_exe_filename(game_title: &str) -> String {
    let ext = if cfg!(windows) { ".exe" } else { "" };
    format!("Uninstall {}{}", sanitize_installer_name(game_title), ext)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ExportBuildKind {
    #[default]
    Portable,
    Installer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomPolicyLanguage {
    pub code: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InstallerConfig {
    pub game_title: String,
    pub version: String,
    pub publisher: String,
    pub welcome_text: String,
    pub license_text: String,
    #[serde(default)]
    pub license_translations: BTreeMap<String, String>,
    #[serde(default)]
    pub custom_policy_languages: Vec<CustomPolicyLanguage>,
    pub default_install_subdir: String,
    pub create_desktop_shortcut_default: bool,
    pub create_start_menu_shortcut_default: bool,
    pub accent_hex: String,
    pub logo_png: Option<Vec<u8>>,
    pub banner_png: Option<Vec<u8>>,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            game_title: "Blanc Visual Novel".to_string(),
            version: "1.0.0".to_string(),
            publisher: "Blanc Studio".to_string(),
            welcome_text: "Welcome to the setup wizard. Click Next to install the game on your computer.".to_string(),
            license_text: String::new(),
            license_translations: BTreeMap::new(),
            custom_policy_languages: Vec::new(),
            default_install_subdir: "BlancVisualNovel".to_string(),
            create_desktop_shortcut_default: true,
            create_start_menu_shortcut_default: true,
            accent_hex: "#6366F1".to_string(),
            logo_png: None,
            banner_png: None,
        }
    }
}

impl InstallerConfig {
    fn apply_placeholders(&self, raw: &str) -> String {
        raw.replace("{GAME_TITLE}", &self.game_title)
            .replace("{PUBLISHER}", &self.publisher)
    }

    pub fn resolve_license_text_by_code(&self, code: &str) -> Option<String> {
        let norm = code.trim().to_ascii_lowercase();
        if let Some(txt) = self.license_translations.get(&norm) {
            if !txt.trim().is_empty() {
                return Some(self.apply_placeholders(txt));
            }
        }
        if norm == "en" && !self.license_text.trim().is_empty() {
            return Some(self.apply_placeholders(&self.license_text));
        }
        if let Some(en_txt) = self.license_translations.get("en") {
            if !en_txt.trim().is_empty() {
                return Some(self.apply_placeholders(en_txt));
            }
        }
        if !self.license_text.trim().is_empty() {
            return Some(self.apply_placeholders(&self.license_text));
        }
        self.license_translations
            .values()
            .find(|v| !v.trim().is_empty())
            .map(|v| self.apply_placeholders(v))
    }

    pub fn resolve_license_text(&self, lang: InstallerLang) -> Option<String> {
        self.resolve_license_text_by_code(lang.code())
    }

    pub fn has_direct_license_for(&self, code: &str) -> bool {
        let norm = code.trim().to_ascii_lowercase();
        self.license_translations
            .get(&norm)
            .is_some_and(|t| !t.trim().is_empty())
            || (norm == "en" && !self.license_text.trim().is_empty())
    }

    pub fn available_policy_languages(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for &lang in &InstallerLang::ALL {
            if self.has_direct_license_for(lang.code()) {
                out.push((lang.code().to_string(), lang.display_name().to_string()));
            }
        }
        for custom in &self.custom_policy_languages {
            let norm = custom.code.trim().to_ascii_lowercase();
            if self.has_direct_license_for(&norm) && !out.iter().any(|(c, _)| c == &norm) {
                out.push((norm, custom.display_name.clone()));
            }
        }
        out
    }

    pub fn has_any_license_text(&self) -> bool {
        !self.license_text.trim().is_empty()
            || self.license_translations.values().any(|v| !v.trim().is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerFileEntry {
    pub relative_path: String,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerPayloadManifest {
    pub config: InstallerConfig,
    pub exe_name: String,
    pub files: Vec<InstallerFileEntry>,
}

/// Build a self-extracting `<Game>_Setup.exe` by appending game files and manifest
/// to the `blanc_installer` stub binary.
///
/// Footer layout:
/// `[stub_exe][file_blobs...][manifest_json][manifest_len: u64 LE][data_start_offset: u64 LE][b"BLNCINST1"]`
pub fn write_installer_exe(
    stub_exe: &[u8],
    output_path: &Path,
    config: &InstallerConfig,
    game_exe_name: &str,
    files: &[(String, Vec<u8>)],
) -> Result<(), String> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create installer output dir: {}", e))?;
    }

    let mut out = File::create(output_path)
        .map_err(|e| format!("Failed to create installer file: {}", e))?;
    out.write_all(stub_exe)
        .map_err(|e| format!("Failed to write installer stub: {}", e))?;

    let data_start_offset = stub_exe.len() as u64;
    let mut cursor: u64 = 0;
    let mut entries = Vec::with_capacity(files.len());

    for (rel_path, bytes) in files {
        out.write_all(bytes)
            .map_err(|e| format!("Failed to write payload file {}: {}", rel_path, e))?;
        entries.push(InstallerFileEntry {
            relative_path: rel_path.replace('\\', "/"),
            offset: cursor,
            size: bytes.len() as u64,
        });
        cursor += bytes.len() as u64;
    }

    let manifest = InstallerPayloadManifest {
        config: config.clone(),
        exe_name: game_exe_name.to_string(),
        files: entries,
    };
    let manifest_bytes = serde_json::to_vec(&manifest)
        .map_err(|e| format!("Failed to serialize installer manifest: {}", e))?;
    let manifest_len = manifest_bytes.len() as u64;

    out.write_all(&manifest_bytes)
        .map_err(|e| format!("Failed to write installer manifest: {}", e))?;
    out.write_all(&manifest_len.to_le_bytes())
        .map_err(|e| format!("Failed to write manifest length: {}", e))?;
    out.write_all(&data_start_offset.to_le_bytes())
        .map_err(|e| format!("Failed to write data start offset: {}", e))?;
    out.write_all(INSTALLER_MAGIC)
        .map_err(|e| format!("Failed to write installer magic: {}", e))?;

    Ok(())
}

/// Read the embedded installer manifest and `data_start_offset` from a self-extracting executable.
pub fn read_embedded_payload(exe_path: &Path) -> Result<(InstallerPayloadManifest, u64), String> {
    let mut f = File::open(exe_path)
        .map_err(|e| format!("Failed to open installer executable: {}", e))?;
    let total_len = f
        .metadata()
        .map_err(|e| format!("Failed to read installer metadata: {}", e))?
        .len();

    let footer_len = 8u64 + 8u64 + INSTALLER_MAGIC.len() as u64;
    if total_len < footer_len {
        return Err("Executable does not contain an installer payload.".to_string());
    }

    f.seek(SeekFrom::End(-(footer_len as i64)))
        .map_err(|e| format!("Failed to seek installer footer: {}", e))?;

    let mut footer = vec![0u8; footer_len as usize];
    f.read_exact(&mut footer)
        .map_err(|e| format!("Failed to read installer footer: {}", e))?;

    if &footer[16..] != INSTALLER_MAGIC {
        return Err("Installer payload magic header not found.".to_string());
    }

    let manifest_len = u64::from_le_bytes(footer[0..8].try_into().unwrap());
    let data_start_offset = u64::from_le_bytes(footer[8..16].try_into().unwrap());

    const MAX_MANIFEST_BYTES: u64 = 64 * 1024 * 1024; // 64 MB cap for manifest JSON
    if data_start_offset > total_len - footer_len
        || manifest_len > total_len - footer_len - data_start_offset
        || manifest_len > MAX_MANIFEST_BYTES
    {
        return Err("Corrupted installer payload offsets.".to_string());
    }

    let manifest_pos = total_len - footer_len - manifest_len;
    f.seek(SeekFrom::Start(manifest_pos))
        .map_err(|e| format!("Failed to seek installer manifest: {}", e))?;
    let manifest_usize = usize::try_from(manifest_len)
        .map_err(|_| "Installer manifest size overflow.".to_string())?;
    let mut manifest_bytes = vec![0u8; manifest_usize];
    f.read_exact(&mut manifest_bytes)
        .map_err(|e| format!("Failed to read installer manifest: {}", e))?;

    let manifest: InstallerPayloadManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| format!("Failed to parse installer manifest: {}", e))?;

    for entry in &manifest.files {
        let Some(end_pos) = data_start_offset
            .checked_add(entry.offset)
            .and_then(|s| s.checked_add(entry.size))
        else {
            return Err("Corrupted installer file entry offset.".to_string());
        };
        if end_pos > manifest_pos {
            return Err(format!(
                "Installer payload entry '{}' exceeds payload bounds.",
                entry.relative_path
            ));
        }
        if crate::project::safe_join_relative(Path::new("."), &entry.relative_path).is_none() {
            return Err(format!(
                "Unsafe relative path in installer payload: '{}'",
                entry.relative_path
            ));
        }
    }

    Ok((manifest, data_start_offset))
}

/// Extract all embedded game files into `dest_dir` and also write a clean `Uninstall.exe`
/// (which is just the stub executable without the embedded payload).
pub fn extract_embedded_files<F>(
    exe_path: &Path,
    dest_dir: &Path,
    mut on_progress: F,
) -> Result<InstallerPayloadManifest, String>
where
    F: FnMut(usize, usize, &str),
{
    let (manifest, data_start_offset) = read_embedded_payload(exe_path)?;
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("Failed to create destination folder: {}", e))?;

    let mut f = File::open(exe_path)
        .map_err(|e| format!("Failed to open installer executable: {}", e))?;

    let total = manifest.files.len();
    let mut installed_files = Vec::with_capacity(total + 1);
    for (idx, entry) in manifest.files.iter().enumerate() {
        on_progress(idx + 1, total, &entry.relative_path);
        let out_path = crate::project::safe_join_relative(dest_dir, &entry.relative_path)
            .ok_or_else(|| format!("Rejected unsafe path in installer: {}", entry.relative_path))?;
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create folder for {}: {}", entry.relative_path, e))?;
        }
        let start = data_start_offset
            .checked_add(entry.offset)
            .ok_or_else(|| format!("Invalid offset for {}", entry.relative_path))?;
        let size = usize::try_from(entry.size)
            .map_err(|_| format!("File size overflow for {}", entry.relative_path))?;
        f.seek(SeekFrom::Start(start))
            .map_err(|e| format!("Failed to seek file {}: {}", entry.relative_path, e))?;
        let mut buf = vec![0u8; size];
        f.read_exact(&mut buf)
            .map_err(|e| format!("Failed to read file {}: {}", entry.relative_path, e))?;
        fs::write(&out_path, &buf)
            .map_err(|e| format!("Failed to write file {}: {}", entry.relative_path, e))?;
        installed_files.push(entry.relative_path.replace('\\', "/"));
    }

    // Write clean per-game uninstaller executable (stub only, without embedded game payload)
    let uninstall_name = uninstall_exe_filename(&manifest.config.game_title);
    let uninstall_path = dest_dir.join(&uninstall_name);
    f.seek(SeekFrom::Start(0))
        .map_err(|e| format!("Failed to seek stub executable: {}", e))?;
    let mut stub_buf = vec![0u8; data_start_offset as usize];
    f.read_exact(&mut stub_buf)
        .map_err(|e| format!("Failed to read stub executable: {}", e))?;
    fs::write(&uninstall_path, &stub_buf)
        .map_err(|e| format!("Failed to write {}: {}", uninstall_name, e))?;

    let uninstall_meta = UninstallMetadata {
        game_title: manifest.config.game_title.clone(),
        version: manifest.config.version.clone(),
        publisher: manifest.config.publisher.clone(),
        game_exe_name: manifest.exe_name.clone(),
        accent_hex: manifest.config.accent_hex.clone(),
        logo_png: manifest.config.logo_png.clone(),
        selected_lang: None,
        installed_files,
    };
    if let Ok(meta_json) = serde_json::to_vec_pretty(&uninstall_meta) {
        let _ = crate::project::atomic_write(&dest_dir.join(UNINSTALL_META_FILENAME), meta_json);
    }

    Ok(manifest)
}
