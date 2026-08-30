use tauri::command;
use tauri_plugin_dialog::DialogExt;
use std::fs::OpenOptions;

use crate::errors::CryptVaultError;
use crate::crypto::kdf::{KdfParams, derive_key};
use crate::crypto::container::{
    create_decoy_volume, add_hidden_volume as add_hidden, open_container as open_cont,
    unseal_header, DECOY_HEADER_OFFSET, HEADER_BLOCK_SIZE
};
use crate::archive::{pack_paths, unpack_to};

#[command]
pub async fn pick_file(app: tauri::AppHandle) -> Result<Option<String>, CryptVaultError> {
    let file_path = app.dialog().file().blocking_pick_file();
    Ok(file_path.map(|p| p.to_string()))
}

#[command]
pub async fn pick_directory(app: tauri::AppHandle) -> Result<Option<String>, CryptVaultError> {
    let folder_path = app.dialog().file().blocking_pick_folder();
    Ok(folder_path.map(|p| p.to_string()))
}

#[command]
pub async fn pick_save_path(app: tauri::AppHandle) -> Result<Option<String>, CryptVaultError> {
    let save_path = app.dialog().file().blocking_save_file();
    Ok(save_path.map(|p| p.to_string()))
}

#[command]
pub async fn create_container(
    path: String,
    total_size_mb: u64,
    decoy_password: String,
    decoy_files: Vec<String>,
    hidden_max_size_mb: Option<u64>,
    kdf_m_cost: Option<u32>,
    kdf_t_cost: Option<u32>,
    kdf_p_cost: Option<u32>,
) -> Result<(), CryptVaultError> {
    let decoy_zip = pack_paths(&decoy_files)?;
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    let mut kdf_params = KdfParams::default();
    if let Some(m) = kdf_m_cost { kdf_params.m_cost = m; }
    if let Some(t) = kdf_t_cost { kdf_params.t_cost = t; }
    if let Some(p) = kdf_p_cost { kdf_params.p_cost = p; }

    let total_size_bytes = total_size_mb * 1024 * 1024;
    let hidden_max_size_bytes = hidden_max_size_mb.unwrap_or(0) * 1024 * 1024;

    create_decoy_volume(
        &mut file,
        total_size_bytes,
        &decoy_zip,
        decoy_password.as_bytes(),
        hidden_max_size_bytes,
        &kdf_params,
    )?;

    Ok(())
}

#[command]
pub async fn add_hidden_volume(
    container_path: String,
    decoy_password: Option<String>,
    hidden_password: String,
    hidden_files: Vec<String>,
    max_hidden_size_mb: u64,
    total_size_mb: u64,
    kdf_m_cost: Option<u32>,
    kdf_t_cost: Option<u32>,
    kdf_p_cost: Option<u32>,
) -> Result<(), CryptVaultError> {
    let hidden_zip = pack_paths(&hidden_files)?;
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&container_path)?;

    let mut kdf_params = KdfParams::default();
    if let Some(m) = kdf_m_cost { kdf_params.m_cost = m; }
    if let Some(t) = kdf_t_cost { kdf_params.t_cost = t; }
    if let Some(p) = kdf_p_cost { kdf_params.p_cost = p; }

    // Calculate decoy_end by decrypting decoy header
    let decoy_end = if let Some(decoy_pw) = decoy_password {
        // Read decoy header block
        let mut decoy_block = vec![0u8; HEADER_BLOCK_SIZE];
        use std::io::{Seek, SeekFrom, Read};
        file.seek(SeekFrom::Start(DECOY_HEADER_OFFSET))?;
        file.read_exact(&mut decoy_block)?;

        let decoy_salt: [u8; 32] = decoy_block[..32]
            .try_into()
            .map_err(|_| CryptVaultError::InvalidContainer)?;

        let decoy_key = derive_key(decoy_pw.as_bytes(), &decoy_salt, &kdf_params)?;
        let decoy_header = unseal_header(&decoy_block, &decoy_key)?;
        
        DECOY_HEADER_OFFSET + HEADER_BLOCK_SIZE as u64 + decoy_header.data_size + 16
    } else {
        return Err(CryptVaultError::DecryptionFailed);
    };

    let total_size_bytes = total_size_mb * 1024 * 1024;
    let max_hidden_size_bytes = max_hidden_size_mb * 1024 * 1024;

    add_hidden(
        &mut file,
        total_size_bytes,
        decoy_end,
        &hidden_zip,
        hidden_password.as_bytes(),
        max_hidden_size_bytes,
        &kdf_params,
    )?;

    Ok(())
}

#[command]
pub async fn open_container(
    container_path: String,
    password: String,
    output_dir: String,
    kdf_m_cost: Option<u32>,
    kdf_t_cost: Option<u32>,
    kdf_p_cost: Option<u32>,
) -> Result<String, CryptVaultError> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&container_path)?;

    let metadata = file.metadata()?;
    let total_size = metadata.len();

    let mut kdf_params = KdfParams::default();
    if let Some(m) = kdf_m_cost { kdf_params.m_cost = m; }
    if let Some(t) = kdf_t_cost { kdf_params.t_cost = t; }
    if let Some(p) = kdf_p_cost { kdf_params.p_cost = p; }

    let result = open_cont(
        &mut file,
        total_size,
        password.as_bytes(),
        &kdf_params,
    )?;

    unpack_to(&result.plaintext, &output_dir)?;

    if result.is_hidden {
        Ok("hidden".into())
    } else {
        Ok("decoy".into())
    }
}
