use crate::migrations;
use crate::models::{Group, Note};
use rusqlite::{params, Connection};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        migrations::initialize(&conn)?;
        Ok(Database { conn: Mutex::new(conn) })
    }

    pub fn create_group(&self, name: &str) -> Result<Group, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        // 获取当前最大 sort_order
        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM groups",
            [],
            |row| row.get(0),
        )?;

        conn.execute(
            "INSERT INTO groups (name, sort_order) VALUES (?1, ?2)",
            params![name, max_order + 1],
        )?;
        let id = conn.last_insert_rowid();
        conn.query_row(
            "SELECT id, name, sort_order, created_at, updated_at FROM groups WHERE id = ?1",
            params![id],
            |row| Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            }),
        )
    }

    pub fn update_group(&self, id: i64, name: &str) -> Result<Group, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE groups SET name = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
            params![name, id],
        )?;
        conn.query_row(
            "SELECT id, name, sort_order, created_at, updated_at FROM groups WHERE id = ?1",
            params![id],
            |row| Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            }),
        )
    }

    pub fn delete_group(&self, id: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM groups WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_all_groups(&self) -> Result<Vec<Group>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, sort_order, created_at, updated_at FROM groups ORDER BY sort_order ASC"
        )?;
        let groups = stmt.query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(groups)
    }

    pub fn get_group(&self, id: i64) -> Result<Option<Group>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, sort_order, created_at, updated_at FROM groups WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            return Ok(Some(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            }));
        }

        Ok(None)
    }

    pub fn update_groups_order(&self, group_ids: Vec<i64>) -> Result<(), rusqlite::Error> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        for (index, id) in group_ids.into_iter().enumerate() {
            tx.execute(
                "UPDATE groups SET sort_order = ?1 WHERE id = ?2",
                params![index as i64, id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn create_note(&self, name: &str, content: &str, group_id: Option<i64>) -> Result<Note, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        
        // Get max sort_order
        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM notes",
            [],
            |row| row.get(0),
        )?;

        conn.execute(
            "INSERT INTO notes (name, content, group_id, sort_order) VALUES (?1, ?2, ?3, ?4)",
            params![name, content, group_id, max_order + 1],
        )?;
        let id = conn.last_insert_rowid();
        conn.query_row(
            "SELECT id, group_id, name, content, sort_order, created_at, updated_at FROM notes WHERE id = ?1",
            params![id],
            |row| Ok(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            }),
        )
    }

    pub fn update_note(&self, id: i64, name: &str, content: &str, group_id: Option<i64>) -> Result<Note, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET name = ?1, content = ?2, group_id = ?3, updated_at = datetime('now', 'localtime') WHERE id = ?4",
            params![name, content, group_id, id],
        )?;
        conn.query_row(
            "SELECT id, group_id, name, content, sort_order, created_at, updated_at FROM notes WHERE id = ?1",
            params![id],
            |row| Ok(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            }),
        )
    }

    pub fn update_notes_order(&self, note_ids: Vec<i64>) -> Result<(), rusqlite::Error> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        for (index, id) in note_ids.into_iter().enumerate() {
            tx.execute(
                "UPDATE notes SET sort_order = ?1 WHERE id = ?2",
                params![index as i64, id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn delete_note(&self, id: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_current_time(&self) -> Result<String, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT datetime('now', 'localtime')", [], |row| row.get(0))
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, group_id, name, content, sort_order, created_at, updated_at FROM notes ORDER BY sort_order ASC"
        )?;
        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(notes)
    }

    pub fn get_note(&self, id: i64) -> Result<Option<Note>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, group_id, name, content, sort_order, created_at, updated_at FROM notes WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            return Ok(Some(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            }));
        }

        Ok(None)
    }

    pub fn import_data(&self, groups: Vec<Group>, notes: Vec<Note>) -> Result<(), rusqlite::Error> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        // Clear existing data
        tx.execute("DELETE FROM notes", [])?;
        tx.execute("DELETE FROM groups", [])?;

        // Map old group IDs to new group IDs
        let mut group_id_map = std::collections::HashMap::new();

        for group in groups {
            tx.execute(
                "INSERT INTO groups (name, sort_order, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                params![group.name, group.sort_order, group.created_at, group.updated_at],
            )?;
            let new_id = tx.last_insert_rowid();
            group_id_map.insert(group.id, new_id);
        }

        for note in notes {
            let new_group_id = note.group_id.and_then(|old_id| group_id_map.get(&old_id).copied());
            tx.execute(
                "INSERT INTO notes (name, content, group_id, sort_order, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![note.name, note.content, new_group_id, note.sort_order, note.created_at, note.updated_at],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use crate::models::{Group, Note};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("s-note-{name}-{unique}.db"))
    }

    #[test]
    fn import_preserves_group_order_and_relationships() {
        let path = temp_db_path("import");
        let db = Database::new(path.to_str().unwrap()).unwrap();

        db.import_data(
            vec![
                Group {
                    id: 101,
                    name: "second".into(),
                    sort_order: 1,
                    created_at: "2026-03-19 10:00:00".into(),
                    updated_at: "2026-03-19 10:00:00".into(),
                },
                Group {
                    id: 100,
                    name: "first".into(),
                    sort_order: 0,
                    created_at: "2026-03-19 09:00:00".into(),
                    updated_at: "2026-03-19 09:00:00".into(),
                },
            ],
            vec![Note {
                id: 1,
                group_id: Some(100),
                name: "note".into(),
                content: "content".into(),
                sort_order: 0,
                created_at: "2026-03-19 11:00:00".into(),
                updated_at: "2026-03-19 11:00:00".into(),
            }],
        )
        .unwrap();

        let groups = db.get_all_groups().unwrap();
        let notes = db.get_all_notes().unwrap();

        assert_eq!(groups.iter().map(|g| g.name.as_str()).collect::<Vec<_>>(), vec!["first", "second"]);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].group_id, Some(groups[0].id));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn update_notes_order_reorders_results() {
        let path = temp_db_path("reorder");
        let db = Database::new(path.to_str().unwrap()).unwrap();

        let first = db.create_note("one", "1", None).unwrap();
        let second = db.create_note("two", "2", None).unwrap();
        let third = db.create_note("three", "3", None).unwrap();

        db.update_notes_order(vec![third.id, first.id, second.id]).unwrap();

        let notes = db.get_all_notes().unwrap();
        assert_eq!(notes.iter().map(|note| note.name.as_str()).collect::<Vec<_>>(), vec!["three", "one", "two"]);

        let _ = fs::remove_file(path);
    }
}
