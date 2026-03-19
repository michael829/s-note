use tauri::{AppHandle, State};
use crate::db::{Database, Group, Note};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub groups: Vec<Group>,
    pub notes: Vec<Note>,
    pub exported_at: String,
}

#[tauri::command]
pub fn import_data(db: State<'_, Database>, json_data: String) -> Result<(), String> {
    let data: ExportData = serde_json::from_str(&json_data).map_err(|e| e.to_string())?;
    db.import_data(data.groups, data.notes).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_group(db: State<'_, Database>, name: String) -> Result<Group, String> {
    db.create_group(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_group(db: State<'_, Database>, id: i64, name: String) -> Result<(), String> {
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
pub fn create_note(db: State<'_, Database>, name: String, content: String, group_id: Option<i64>) -> Result<Note, String> {
    db.create_note(&name, &content, group_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(db: State<'_, Database>, id: i64, name: String, content: String, group_id: Option<i64>) -> Result<(), String> {
    db.update_note(id, &name, &content, group_id).map_err(|e| e.to_string())
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
    let groups = db.get_all_groups().map_err(|e| e.to_string())?;
    let notes = db.get_all_notes().map_err(|e| e.to_string())?;
    let data = ExportData {
        groups,
        notes,
        exported_at: db.get_current_time().unwrap_or_default(),
    };
    serde_json::to_string_pretty(&data).map_err(|e| e.to_string())
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
