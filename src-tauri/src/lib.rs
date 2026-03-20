mod app_state;
mod bootstrap;
mod commands;
mod db;
mod migrations;
mod models;
mod panel;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| bootstrap::setup(app))
        .invoke_handler(tauri::generate_handler![
            commands::create_group,
            commands::update_group,
            commands::delete_group,
            commands::get_all_groups,
            commands::get_group,
            commands::create_note,
            commands::update_note,
            commands::get_note,
            commands::update_notes_order,
            commands::update_groups_order,
            commands::delete_note,
            commands::get_all_notes,
            commands::copy_to_clipboard,
            commands::export_data,
            commands::import_data,
            commands::save_to_file,
            commands::read_file,
            commands::get_app_version,
            commands::quit_app,
            commands::set_ignore_blur,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
