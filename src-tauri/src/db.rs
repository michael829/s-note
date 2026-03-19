use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: i64,
    pub group_id: Option<i64>,
    pub name: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS groups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT (datetime('now', 'localtime')),
                updated_at DATETIME DEFAULT (datetime('now', 'localtime'))
            );
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                group_id INTEGER NULL,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT (datetime('now', 'localtime')),
                updated_at DATETIME DEFAULT (datetime('now', 'localtime')),
                FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
            );"
        )?;
        Ok(Database { conn: Mutex::new(conn) })
    }

    pub fn create_group(&self, name: &str) -> Result<Group, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT INTO groups (name) VALUES (?1)", params![name])?;
        let id = conn.last_insert_rowid();
        conn.query_row(
            "SELECT id, name, created_at, updated_at FROM groups WHERE id = ?1",
            params![id],
            |row| Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            }),
        )
    }

    pub fn update_group(&self, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE groups SET name = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
            params![name, id],
        )?;
        Ok(())
    }

    pub fn delete_group(&self, id: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM groups WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_all_groups(&self) -> Result<Vec<Group>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, created_at, updated_at FROM groups ORDER BY created_at DESC"
        )?;
        let groups = stmt.query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(groups)
    }

    pub fn create_note(&self, name: &str, content: &str, group_id: Option<i64>) -> Result<Note, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO notes (name, content, group_id) VALUES (?1, ?2, ?3)",
            params![name, content, group_id],
        )?;
        let id = conn.last_insert_rowid();
        conn.query_row(
            "SELECT id, group_id, name, content, created_at, updated_at FROM notes WHERE id = ?1",
            params![id],
            |row| Ok(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            }),
        )
    }

    pub fn update_note(&self, id: i64, name: &str, content: &str, group_id: Option<i64>) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET name = ?1, content = ?2, group_id = ?3, updated_at = datetime('now', 'localtime') WHERE id = ?4",
            params![name, content, group_id, id],
        )?;
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
            "SELECT id, group_id, name, content, created_at, updated_at FROM notes ORDER BY created_at DESC"
        )?;
        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(notes)
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
                "INSERT INTO groups (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
                params![group.name, group.created_at, group.updated_at],
            )?;
            let new_id = tx.last_insert_rowid();
            group_id_map.insert(group.id, new_id);
        }

        for note in notes {
            let new_group_id = note.group_id.and_then(|old_id| group_id_map.get(&old_id).copied());
            tx.execute(
                "INSERT INTO notes (name, content, group_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![note.name, note.content, new_group_id, note.created_at, note.updated_at],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}
