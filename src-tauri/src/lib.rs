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
            commands::get_recent_archives,
            commands::remove_from_recent,
            commands::create_archive,
            commands::add_alt_password,
            commands::open_archive,
            commands::add_to_archive,
            commands::extract_archive,
            commands::extract_files,
            commands::pick_files,
            commands::pick_directory,
            commands::pick_open_qbv,
            commands::pick_save_qbv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
