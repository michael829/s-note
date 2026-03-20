use crate::db::Database;
use crate::models::{Group, Note};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const EXPORT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub version: u32,
    pub groups: Vec<Group>,
    pub notes: Vec<Note>,
    pub exported_at: String,
}

pub fn import_data(db: &Database, json_data: &str) -> Result<(), String> {
    let data: ExportData = serde_json::from_str(json_data).map_err(|e| e.to_string())?;
    validate_import_data(&data)?;
    db.import_data(data.groups, data.notes).map_err(|e| e.to_string())
}

pub fn export_data(db: &Database) -> Result<String, String> {
    let data = ExportData {
        version: EXPORT_VERSION,
        groups: db.get_all_groups().map_err(|e| e.to_string())?,
        notes: db.get_all_notes().map_err(|e| e.to_string())?,
        exported_at: db.get_current_time().unwrap_or_default(),
    };

    serde_json::to_string_pretty(&data).map_err(|e| e.to_string())
}

pub fn update_notes_order(db: &Database, note_ids: Vec<i64>) -> Result<(), String> {
    db.update_notes_order(note_ids).map_err(|e| e.to_string())
}

pub fn update_groups_order(db: &Database, group_ids: Vec<i64>) -> Result<(), String> {
    db.update_groups_order(group_ids).map_err(|e| e.to_string())
}

fn validate_import_data(data: &ExportData) -> Result<(), String> {
    if data.version != EXPORT_VERSION {
        return Err(format!(
            "Unsupported import version {}. Expected version {}.",
            data.version, EXPORT_VERSION
        ));
    }

    let mut group_ids = HashSet::new();
    let mut group_names = HashSet::new();
    for group in &data.groups {
        if group.name.trim().is_empty() {
            return Err("Group name cannot be empty.".into());
        }
        if !group_ids.insert(group.id) {
            return Err(format!("Duplicate group id {} found in import file.", group.id));
        }
        if !group_names.insert(group.name.trim().to_string()) {
            return Err(format!("Duplicate group name '{}' found in import file.", group.name));
        }
    }

    let mut note_ids = HashSet::new();
    for note in &data.notes {
        if note.name.trim().is_empty() {
            return Err("Note title cannot be empty.".into());
        }
        if note.content.trim().is_empty() {
            return Err("Note content cannot be empty.".into());
        }
        if !note_ids.insert(note.id) {
            return Err(format!("Duplicate note id {} found in import file.", note.id));
        }
        if let Some(group_id) = note.group_id {
            if !group_ids.contains(&group_id) {
                return Err(format!(
                    "Note '{}' references missing group id {}.",
                    note.name, group_id
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_import_data, ExportData, EXPORT_VERSION};
    use crate::models::{Group, Note};

    fn valid_payload() -> ExportData {
        ExportData {
            version: EXPORT_VERSION,
            exported_at: "2026-03-20 10:00:00".into(),
            groups: vec![Group {
                id: 1,
                name: "Group A".into(),
                sort_order: 0,
                created_at: "2026-03-20 10:00:00".into(),
                updated_at: "2026-03-20 10:00:00".into(),
            }],
            notes: vec![Note {
                id: 2,
                group_id: Some(1),
                name: "Note A".into(),
                content: "Body".into(),
                sort_order: 0,
                created_at: "2026-03-20 10:00:00".into(),
                updated_at: "2026-03-20 10:00:00".into(),
            }],
        }
    }

    #[test]
    fn import_validation_rejects_bad_version() {
        let mut payload = valid_payload();
        payload.version = 999;
        let error = validate_import_data(&payload).unwrap_err();
        assert!(error.contains("Unsupported import version"));
    }

    #[test]
    fn import_validation_rejects_missing_group_reference() {
        let mut payload = valid_payload();
        payload.notes[0].group_id = Some(99);
        let error = validate_import_data(&payload).unwrap_err();
        assert!(error.contains("references missing group id"));
    }
}
