use crate::errors::CryptVaultError;

#[tauri::command]
pub async fn create_container(
    _path: String,
    _total_size_mb: u64,
    _decoy_password: String,
    _decoy_files: Vec<String>,
    _kdf_m_cost: Option<u32>,
    _kdf_t_cost: Option<u32>,
    _kdf_p_cost: Option<u32>,
) -> Result<(), CryptVaultError> {
    Err(CryptVaultError::InvalidContainer) // placeholder until commit 7
}

#[tauri::command]
pub async fn add_hidden_volume(
    _container_path: String,
    _hidden_password: String,
    _hidden_files: Vec<String>,
    _max_hidden_size_mb: u64,
) -> Result<(), CryptVaultError> {
    Err(CryptVaultError::InvalidContainer) // placeholder until commit 8
}

#[tauri::command]
pub async fn open_container(
    _container_path: String,
    _password: String,
    _output_dir: String,
) -> Result<String, CryptVaultError> {
    Err(CryptVaultError::InvalidContainer) // placeholder until commit 10
}

#[tauri::command]
pub async fn pick_file() -> Result<Option<String>, CryptVaultError> {
    Ok(None) // placeholder until commit 7
}

#[tauri::command]
pub async fn pick_directory() -> Result<Option<String>, CryptVaultError> {
    Ok(None) // placeholder until commit 7
}

#[tauri::command]
pub async fn pick_save_path() -> Result<Option<String>, CryptVaultError> {
    Ok(None) // placeholder until commit 7
}
