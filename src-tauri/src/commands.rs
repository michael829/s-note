use crate::db::Database;
use crate::models::{Group, Note};
use crate::service;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn import_data(db: State<'_, Database>, json_data: String) -> Result<(), String> {
    service::import_data(&db, &json_data)
}

#[tauri::command]
pub fn create_group(db: State<'_, Database>, name: String) -> Result<Group, String> {
    db.create_group(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_group(db: State<'_, Database>, id: i64, name: String) -> Result<Group, String> {
    db.update_group(id, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_group(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_group(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_groups(db: State<'_, Database>) -> Result<Vec<Group>, String> {
    db.get_all_groups().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_group(db: State<'_, Database>, id: i64) -> Result<Option<Group>, String> {
    db.get_group(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_note(db: State<'_, Database>, name: String, content: String, group_id: Option<i64>) -> Result<Note, String> {
    db.create_note(&name, &content, group_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(db: State<'_, Database>, id: i64, name: String, content: String, group_id: Option<i64>) -> Result<Note, String> {
    db.update_note(id, &name, &content, group_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(db: State<'_, Database>, id: i64) -> Result<Option<Note>, String> {
    db.get_note(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_notes_order(db: State<'_, Database>, note_ids: Vec<i64>) -> Result<(), String> {
    service::update_notes_order(&db, note_ids)
}

#[tauri::command]
pub fn update_groups_order(db: State<'_, Database>, group_ids: Vec<i64>) -> Result<(), String> {
    service::update_groups_order(&db, group_ids)
}

#[tauri::command]
pub fn delete_note(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_note(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_notes(db: State<'_, Database>) -> Result<Vec<Note>, String> {
    db.get_all_notes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn copy_to_clipboard(content: String) -> Result<(), String> {
    use arboard::Clipboard;
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_data(db: State<'_, Database>) -> Result<String, String> {
    service::export_data(&db)
}

#[tauri::command]
pub fn save_to_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn set_ignore_blur(ignore: bool) {
    crate::app_state::IGNORE_BLUR.store(ignore, std::sync::atomic::Ordering::SeqCst);
}
