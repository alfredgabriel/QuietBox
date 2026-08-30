pub mod crypto;
pub mod commands;
pub mod fs_ops;
pub mod errors;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::create_container,
            commands::add_hidden_volume,
            commands::open_container,
            commands::pick_file,
            commands::pick_directory,
            commands::pick_save_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
