pub mod crypto;
pub mod commands;
pub mod fs_ops;
pub mod errors;
pub mod archive;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_vaults,
            commands::create_vault,
            commands::update_vault_position,
            commands::rename_vault,
            commands::delete_vault,
            commands::open_vault_by_id,
            commands::add_extra_password,
            commands::import_files_to_vault,
            commands::export_vault_to_dir,
            commands::pick_file,
            commands::pick_directory,
            commands::pick_save_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
