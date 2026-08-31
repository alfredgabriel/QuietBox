use tauri::{command, AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;
use std::fs::{self, OpenOptions, File};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::errors::CryptVaultError;
use crate::crypto::kdf::KdfParams;
use crate::crypto::container::{
    create_decoy_volume, add_hidden_volume as add_hidden, open_container as open_cont,
};
use crate::archive::{
    unpack_to, extract_entries, remove_from_zip, create_initial_zip, list_zip_entries, append_to_zip,
    pack_paths, VaultFileEntry,
};

const HIDDEN_RESERVED: u64 = 10 * 1024 * 1024; // 10 MB reserved for alt space

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentArchive {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub opened_at: u64,
    pub has_alt_key: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenArchiveResult {
    pub path: String,
    pub name: String,
    pub is_alt: bool,
    pub entries: Vec<VaultFileEntry>,
    pub has_alt_key: bool,
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
    progress: f64,
    status: String,
}

fn app_data_dir() -> PathBuf {
    let mut d = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    d.push("QuietBox");
    let _ = fs::create_dir_all(&d);
    d
}

fn recent_path() -> PathBuf {
    app_data_dir().join("recent.json")
}

fn read_recent() -> Vec<RecentArchive> {
    fs::read_to_string(recent_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_recent(list: &[RecentArchive]) {
    if let Ok(json) = serde_json::to_string_pretty(list) {
        let _ = fs::write(recent_path(), json);
    }
}

fn push_recent(path: &str, name: &str, has_alt_key: bool) {
    let mut list = read_recent();
    list.retain(|r| r.path != path);
    let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    list.insert(0, RecentArchive {
        path: path.to_string(),
        name: name.to_string(),
        size_bytes: size,
        opened_at: now,
        has_alt_key,
    });
    list.truncate(20);
    save_recent(&list);
}

fn emit_progress(app: &AppHandle, p: f64, s: &str) {
    let _ = app.emit("progress", ProgressPayload { progress: p, status: s.to_string() });
}

// ── Commands ─────────────────────────────────────────────────────────────

#[command]
pub async fn get_recent_archives() -> Result<Vec<RecentArchive>, CryptVaultError> {
    Ok(read_recent()
        .into_iter()
        .filter(|r| std::path::Path::new(&r.path).exists())
        .collect())
}

#[command]
pub async fn remove_from_recent(path: String) -> Result<(), CryptVaultError> {
    let mut list = read_recent();
    list.retain(|r| r.path != path);
    save_recent(&list);
    Ok(())
}

/// Create a new encrypted archive from a list of files.
/// Returns the path to the created .qbv file.
#[command]
pub async fn create_archive(
    app: AppHandle,
    name: String,
    files: Vec<String>,
    password: String,
    output_path: String,
) -> Result<String, CryptVaultError> {
    emit_progress(&app, 0.05, "Packing files…");
    let zip_bytes = if files.is_empty() {
        create_initial_zip(&name)?
    } else {
        pack_paths(&files)?
    };

    emit_progress(&app, 0.20, "Generating encryption keys…");

    // Container size: zip * 4 + generous padding, minimum 100 MB
    let min_size: u64 = 100 * 1024 * 1024;
    let content_size = (zip_bytes.len() as u64 * 4).max(min_size);
    // Round up to nearest 4 MB block
    let block = 4 * 1024 * 1024_u64;
    let total_size = ((content_size + block - 1) / block) * block;

    let app_clone = app.clone();
    let on_progress = move |p: f64, s: &str| emit_progress(&app_clone, 0.20 + p * 0.75, s);

    let mut file = OpenOptions::new()
        .read(true).write(true).create(true).truncate(true)
        .open(&output_path)?;

    let kdf = KdfParams::default();
    create_decoy_volume(&mut file, total_size, &zip_bytes, password.as_bytes(),
                        HIDDEN_RESERVED, &kdf, on_progress)?;

    emit_progress(&app, 1.0, "Done");
    push_recent(&output_path, &name, false);
    Ok(output_path)
}

/// Add an alternative password with its own set of files to an existing archive.
#[command]
pub async fn add_alt_password(
    app: AppHandle,
    archive_path: String,
    alt_password: String,
    alt_files: Vec<String>,
) -> Result<(), CryptVaultError> {
    emit_progress(&app, 0.05, "Packing alternative files…");
    let zip_bytes = if alt_files.is_empty() {
        create_initial_zip("Alternative")?
    } else {
        pack_paths(&alt_files)?
    };

    emit_progress(&app, 0.20, "Generating alternative encryption keys…");

    let app_clone = app.clone();
    let on_progress = move |p: f64, s: &str| emit_progress(&app_clone, 0.20 + p * 0.75, s);

    let mut file = OpenOptions::new().read(true).write(true).open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();

    add_hidden(&mut file, total_size, 0, &zip_bytes, alt_password.as_bytes(),
               HIDDEN_RESERVED, &kdf, on_progress)?;

    emit_progress(&app, 1.0, "Done");

    // Mark as having alt key in recent
    let name = std::path::Path::new(&archive_path)
        .file_stem().and_then(|s| s.to_str()).unwrap_or("archive").to_string();
    push_recent(&archive_path, &name, true);
    Ok(())
}

/// Open an archive with the given password. Works with both primary and alternative passwords.
#[command]
pub async fn open_archive(
    app: AppHandle,
    archive_path: String,
    password: String,
) -> Result<OpenArchiveResult, CryptVaultError> {
    emit_progress(&app, 0.1, "Reading archive…");
    let mut file = File::open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();

    emit_progress(&app, 0.4, "Decrypting…");
    let result = open_cont(&mut file, total_size, password.as_bytes(), &kdf)?;

    emit_progress(&app, 0.8, "Reading file list…");
    let entries = list_zip_entries(&result.plaintext)?;

    let name = std::path::Path::new(&archive_path)
        .file_stem().and_then(|s| s.to_str()).unwrap_or("archive").to_string();

    // check if alt key exists (best effort)
    let recent = read_recent();
    let has_alt = recent.iter().any(|r| r.path == archive_path && r.has_alt_key);

    push_recent(&archive_path, &name, has_alt);
    emit_progress(&app, 1.0, "Ready");

    Ok(OpenArchiveResult {
        path: archive_path,
        name,
        is_alt: result.is_hidden,
        entries,
        has_alt_key: has_alt,
    })
}

/// Add more files to an already-opened archive (re-encrypts with same password).
#[command]
pub async fn add_to_archive(
    app: AppHandle,
    archive_path: String,
    password: String,
    new_files: Vec<String>,
) -> Result<Vec<VaultFileEntry>, CryptVaultError> {
    emit_progress(&app, 0.05, "Opening archive…");
    let mut file = OpenOptions::new().read(true).write(true).open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();

    let result = open_cont(&mut file, total_size, password.as_bytes(), &kdf)?;
    emit_progress(&app, 0.30, "Appending files…");
    let updated_zip = append_to_zip(&result.plaintext, &new_files)?;

    let app_clone = app.clone();
    let on_progress = move |p: f64, s: &str| emit_progress(&app_clone, 0.30 + p * 0.65, s);

    if result.is_hidden {
        add_hidden(&mut file, total_size, 0, &updated_zip, password.as_bytes(),
                   HIDDEN_RESERVED, &kdf, on_progress)?;
    } else {
        create_decoy_volume(&mut file, total_size, &updated_zip, password.as_bytes(),
                            HIDDEN_RESERVED, &kdf, on_progress)?;
    }

    emit_progress(&app, 1.0, "Done");
    list_zip_entries(&updated_zip).map_err(Into::into)
}

/// Extract all files from the archive to a target directory.
#[command]
pub async fn extract_archive(
    app: AppHandle,
    archive_path: String,
    password: String,
    output_dir: String,
) -> Result<(), CryptVaultError> {
    emit_progress(&app, 0.1, "Decrypting archive…");
    let mut file = File::open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();

    let result = open_cont(&mut file, total_size, password.as_bytes(), &kdf)?;
    emit_progress(&app, 0.7, "Extracting files…");
    unpack_to(&result.plaintext, &output_dir)?;
    emit_progress(&app, 1.0, "Done");
    Ok(())
}


/// Extract specific selected files from an archive.
#[command]
pub async fn extract_files(
    app: AppHandle,
    archive_path: String,
    password: String,
    entry_paths: Vec<String>,
    output_dir: String,
) -> Result<(), CryptVaultError> {
    emit_progress(&app, 0.1, "Decrypting...");
    let mut file = File::open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();
    let result = open_cont(&mut file, total_size, password.as_bytes(), &kdf)?;
    emit_progress(&app, 0.7, "Extracting selected files...");
    extract_entries(&result.plaintext, &entry_paths, &output_dir)?;
    emit_progress(&app, 1.0, "Done");
    Ok(())
}


/// Delete specific files from an archive.
#[command]
pub async fn delete_from_archive(
    app: AppHandle,
    archive_path: String,
    password: String,
    entry_paths: Vec<String>,
) -> Result<Vec<VaultFileEntry>, CryptVaultError> {
    emit_progress(&app, 0.1, "Opening archive…");
    let mut file = OpenOptions::new().read(true).write(true).open(&archive_path)?;
    let total_size = file.metadata()?.len();
    let kdf = KdfParams::default();

    let result = open_cont(&mut file, total_size, password.as_bytes(), &kdf)?;
    emit_progress(&app, 0.4, "Removing selected files…");
    let updated_zip = remove_from_zip(&result.plaintext, &entry_paths)?;

    let app_clone = app.clone();
    let on_progress = move |p: f64, s: &str| emit_progress(&app_clone, 0.40 + p * 0.55, s);

    if result.is_hidden {
        add_hidden(&mut file, total_size, 0, &updated_zip, password.as_bytes(),
                   HIDDEN_RESERVED, &kdf, on_progress)?;
    } else {
        create_decoy_volume(&mut file, total_size, &updated_zip, password.as_bytes(),
                            HIDDEN_RESERVED, &kdf, on_progress)?;
    }

    emit_progress(&app, 1.0, "Done");
    list_zip_entries(&updated_zip).map_err(Into::into)
}

// ── File pickers ──────────────────────────────────────────────────────────

#[command]
pub async fn pick_files(app: AppHandle) -> Result<Vec<String>, CryptVaultError> {
    let result = app.dialog().file().blocking_pick_files();
    Ok(result.map(|v| v.into_iter().map(|p| p.to_string()).collect()).unwrap_or_default())
}

#[command]
pub async fn pick_directory(app: AppHandle) -> Result<Option<String>, CryptVaultError> {
    Ok(app.dialog().file().blocking_pick_folder().map(|p| p.to_string()))
}

#[command]
pub async fn pick_open_qbv(app: AppHandle) -> Result<Option<String>, CryptVaultError> {
    Ok(app.dialog().file()
        .add_filter("QuietBox Archive", &["qbv"])
        .blocking_pick_file()
        .map(|p| p.to_string()))
}

#[command]
pub async fn pick_save_qbv(app: AppHandle, default_name: String) -> Result<Option<String>, CryptVaultError> {
    Ok(app.dialog().file()
        .add_filter("QuietBox Archive", &["qbv"])
        .set_file_name(&format!("{}.qbv", default_name))
        .blocking_save_file()
        .map(|p| p.to_string()))
}
