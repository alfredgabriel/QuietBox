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
    unpack_to, create_initial_zip, list_zip_entries, append_to_zip, VaultFileEntry
};

const DEFAULT_VAULT_SIZE_BYTES: u64 = 50 * 1024 * 1024; // 50 MB initial fixed size

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultMetadata {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub grid_x: u32,
    pub grid_y: u32,
    pub created_at: u64,
    pub has_extra_key: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultItem {
    pub id: String,
    pub name: String,
    pub path: String,
    pub grid_x: u32,
    pub grid_y: u32,
    pub size_bytes: u64,
    pub created_at: u64,
    pub has_extra_key: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenVaultResponse {
    pub id: String,
    pub is_extra: bool,
    pub entries: Vec<VaultFileEntry>,
}

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    progress: f64,
    status: String,
}

fn get_app_storage_dir() -> PathBuf {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("QuietBox");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn get_vaults_dir() -> PathBuf {
    let mut dir = get_app_storage_dir();
    dir.push("vaults");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn get_metadata_path() -> PathBuf {
    let mut path = get_app_storage_dir();
    path.push("vaults_meta.json");
    path
}

fn read_metadata() -> Vec<VaultMetadata> {
    let path = get_metadata_path();
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_metadata(meta: &[VaultMetadata]) {
    let path = get_metadata_path();
    if let Ok(json) = serde_json::to_string_pretty(meta) {
        let _ = fs::write(path, json);
    }
}

#[command]
pub async fn list_vaults() -> Result<Vec<VaultItem>, CryptVaultError> {
    let meta = read_metadata();
    let vaults_dir = get_vaults_dir();
    let mut items = Vec::new();

    for m in meta {
        let path = vaults_dir.join(&m.filename);
        if path.exists() {
            let size = path.metadata().map(|md| md.len()).unwrap_or(0);
            items.push(VaultItem {
                id: m.id,
                name: m.name,
                path: path.to_string_lossy().to_string(),
                grid_x: m.grid_x,
                grid_y: m.grid_y,
                size_bytes: size,
                created_at: m.created_at,
                has_extra_key: m.has_extra_key,
            });
        }
    }

    Ok(items)
}

#[command]
pub async fn create_vault(
    app: AppHandle,
    name: String,
    password: String,
    grid_x: Option<u32>,
    grid_y: Option<u32>,
) -> Result<VaultItem, CryptVaultError> {
    let id = format!("vault_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
    let filename = format!("{}.qbv", id);
    let vault_path = get_vaults_dir().join(&filename);

    let initial_zip = create_initial_zip(&name)?;

    let app_clone = app.clone();
    let on_progress = move |progress: f64, status: &str| {
        let _ = app_clone.emit("progress", ProgressPayload {
            progress,
            status: status.to_string(),
        });
    };

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&vault_path)?;

    let kdf_params = KdfParams::default();
    let hidden_reserved_size = 15 * 1024 * 1024; // 15 MB reserved for extra key space

    create_decoy_volume(
        &mut file,
        DEFAULT_VAULT_SIZE_BYTES,
        &initial_zip,
        password.as_bytes(),
        hidden_reserved_size,
        &kdf_params,
        on_progress,
    )?;

    // Calculate grid placement
    let mut meta = read_metadata();
    let gx = grid_x.unwrap_or((meta.len() as u32 % 5) * 1);
    let gy = grid_y.unwrap_or((meta.len() as u32 / 5) * 1);

    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    let new_meta = VaultMetadata {
        id: id.clone(),
        name: name.clone(),
        filename,
        grid_x: gx,
        grid_y: gy,
        created_at: now,
        has_extra_key: false,
    };

    meta.push(new_meta);
    save_metadata(&meta);

    Ok(VaultItem {
        id,
        name,
        path: vault_path.to_string_lossy().to_string(),
        grid_x: gx,
        grid_y: gy,
        size_bytes: DEFAULT_VAULT_SIZE_BYTES,
        created_at: now,
        has_extra_key: false,
    })
}

#[command]
pub async fn update_vault_position(id: String, grid_x: u32, grid_y: u32) -> Result<(), CryptVaultError> {
    let mut meta = read_metadata();
    if let Some(item) = meta.iter_mut().find(|m| m.id == id) {
        item.grid_x = grid_x;
        item.grid_y = grid_y;
        save_metadata(&meta);
    }
    Ok(())
}

#[command]
pub async fn rename_vault(id: String, new_name: String) -> Result<(), CryptVaultError> {
    let mut meta = read_metadata();
    if let Some(item) = meta.iter_mut().find(|m| m.id == id) {
        item.name = new_name;
        save_metadata(&meta);
    }
    Ok(())
}

#[command]
pub async fn delete_vault(id: String) -> Result<(), CryptVaultError> {
    let mut meta = read_metadata();
    if let Some(idx) = meta.iter().position(|m| m.id == id) {
        let item = meta.remove(idx);
        let path = get_vaults_dir().join(&item.filename);
        let _ = fs::remove_file(path);
        save_metadata(&meta);
    }
    Ok(())
}

#[command]
pub async fn open_vault_by_id(
    id: String,
    password: String,
) -> Result<OpenVaultResponse, CryptVaultError> {
    let meta = read_metadata();
    let item = meta.iter().find(|m| m.id == id).ok_or(CryptVaultError::InvalidContainer)?;
    let path = get_vaults_dir().join(&item.filename);

    let mut file = File::open(&path)?;
    let total_size = file.metadata()?.len();
    let kdf_params = KdfParams::default();

    let open_res = open_cont(&mut file, total_size, password.as_bytes(), &kdf_params)?;
    let entries = list_zip_entries(&open_res.plaintext)?;

    Ok(OpenVaultResponse {
        id,
        is_extra: open_res.is_hidden,
        entries,
    })
}

#[command]
pub async fn add_extra_password(
    app: AppHandle,
    id: String,
    extra_password: String,
) -> Result<(), CryptVaultError> {
    let mut meta = read_metadata();
    let item = meta.iter_mut().find(|m| m.id == id).ok_or(CryptVaultError::InvalidContainer)?;
    let path = get_vaults_dir().join(&item.filename);

    let app_clone = app.clone();
    let on_progress = move |progress: f64, status: &str| {
        let _ = app_clone.emit("progress", ProgressPayload {
            progress,
            status: status.to_string(),
        });
    };

    let initial_zip = create_initial_zip("Confidential Space")?;
    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
    let total_size = file.metadata()?.len();
    let kdf_params = KdfParams::default();
    let hidden_max_size = 15 * 1024 * 1024;

    add_hidden(
        &mut file,
        total_size,
        0,
        &initial_zip,
        extra_password.as_bytes(),
        hidden_max_size,
        &kdf_params,
        on_progress,
    )?;

    item.has_extra_key = true;
    save_metadata(&meta);

    Ok(())
}

#[command]
pub async fn import_files_to_vault(
    app: AppHandle,
    id: String,
    password: String,
    file_paths: Vec<String>,
) -> Result<Vec<VaultFileEntry>, CryptVaultError> {
    let meta = read_metadata();
    let item = meta.iter().find(|m| m.id == id).ok_or(CryptVaultError::InvalidContainer)?;
    let path = get_vaults_dir().join(&item.filename);

    let app_clone = app.clone();
    let on_progress = move |progress: f64, status: &str| {
        let _ = app_clone.emit("progress", ProgressPayload {
            progress,
            status: status.to_string(),
        });
    };

    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
    let total_size = file.metadata()?.len();
    let kdf_params = KdfParams::default();

    let open_res = open_cont(&mut file, total_size, password.as_bytes(), &kdf_params)?;
    let updated_zip = append_to_zip(&open_res.plaintext, &file_paths)?;

    if open_res.is_hidden {
        let hidden_max_size = 15 * 1024 * 1024;
        add_hidden(
            &mut file,
            total_size,
            0,
            &updated_zip,
            password.as_bytes(),
            hidden_max_size,
            &kdf_params,
            on_progress,
        )?;
    } else {
        create_decoy_volume(
            &mut file,
            total_size,
            &updated_zip,
            password.as_bytes(),
            15 * 1024 * 1024,
            &kdf_params,
            on_progress,
        )?;
    }

    let entries = list_zip_entries(&updated_zip)?;
    Ok(entries)
}

#[command]
pub async fn export_vault_to_dir(
    id: String,
    password: String,
    output_dir: String,
) -> Result<(), CryptVaultError> {
    let meta = read_metadata();
    let item = meta.iter().find(|m| m.id == id).ok_or(CryptVaultError::InvalidContainer)?;
    let path = get_vaults_dir().join(&item.filename);

    let mut file = File::open(&path)?;
    let total_size = file.metadata()?.len();
    let kdf_params = KdfParams::default();

    let open_res = open_cont(&mut file, total_size, password.as_bytes(), &kdf_params)?;
    unpack_to(&open_res.plaintext, &output_dir)?;

    Ok(())
}

#[command]
pub async fn pick_file(app: AppHandle) -> Result<Option<String>, CryptVaultError> {
    let file_path = app.dialog().file().blocking_pick_file();
    Ok(file_path.map(|p| p.to_string()))
}

#[command]
pub async fn pick_directory(app: AppHandle) -> Result<Option<String>, CryptVaultError> {
    let folder_path = app.dialog().file().blocking_pick_folder();
    Ok(folder_path.map(|p| p.to_string()))
}

#[command]
pub async fn pick_save_path(app: AppHandle) -> Result<Option<String>, CryptVaultError> {
    let save_path = app.dialog().file().blocking_save_file();
    Ok(save_path.map(|p| p.to_string()))
}
