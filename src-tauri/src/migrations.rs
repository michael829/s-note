use rusqlite::Connection;

pub fn initialize(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL CHECK (length(trim(name)) > 0),
            sort_order INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_at DATETIME DEFAULT (datetime('now', 'localtime'))
        );
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NULL,
            name TEXT NOT NULL CHECK (length(trim(name)) > 0),
            content TEXT NOT NULL CHECK (length(trim(content)) > 0),
            sort_order INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_at DATETIME DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
        );"
    )?;

    let has_group_sort_order = has_column(conn, "groups", "sort_order")?;
    let has_note_sort_order = has_column(conn, "notes", "sort_order")?;

    if !has_group_sort_order {
        conn.execute_batch("ALTER TABLE groups ADD COLUMN sort_order INTEGER DEFAULT 0;")?;
        conn.execute_batch(
            "UPDATE groups SET sort_order = (
                SELECT COUNT(*) FROM groups g2 WHERE g2.created_at > groups.created_at
            );"
        )?;
    }

    if !has_note_sort_order {
        conn.execute_batch("ALTER TABLE notes ADD COLUMN sort_order INTEGER DEFAULT 0;")?;
        conn.execute_batch(
            "UPDATE notes SET sort_order = (
                SELECT COUNT(*) FROM notes n2 WHERE n2.created_at > notes.created_at
            );"
        )?;
    }

    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_groups_sort_order ON groups(sort_order);
        CREATE INDEX IF NOT EXISTS idx_notes_sort_order ON notes(sort_order);
        CREATE INDEX IF NOT EXISTS idx_notes_group_sort_order ON notes(group_id, sort_order);"
    )?;

    Ok(())
}

fn has_column(conn: &Connection, table: &str, column: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(columns.iter().any(|c| c == column))
}

#[cfg(test)]
mod tests {
    use super::initialize;
    use rusqlite::Connection;

    #[test]
    fn migrates_legacy_schema_and_preserves_ordering_data() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE groups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME,
                updated_at DATETIME
            );
            CREATE TABLE notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                group_id INTEGER NULL,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME,
                updated_at DATETIME
            );
            INSERT INTO groups (id, name, created_at, updated_at) VALUES
              (1, 'older', '2026-03-18 09:00:00', '2026-03-18 09:00:00'),
              (2, 'newer', '2026-03-19 09:00:00', '2026-03-19 09:00:00');
            INSERT INTO notes (id, group_id, name, content, created_at, updated_at) VALUES
              (1, 1, 'first', 'A', '2026-03-18 10:00:00', '2026-03-18 10:00:00'),
              (2, 2, 'second', 'B', '2026-03-19 10:00:00', '2026-03-19 10:00:00');"
        ).unwrap();

        initialize(&conn).unwrap();

        let group_columns: Vec<String> = conn
            .prepare("PRAGMA table_info(groups)")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        let note_columns: Vec<String> = conn
            .prepare("PRAGMA table_info(notes)")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert!(group_columns.contains(&"sort_order".to_string()));
        assert!(note_columns.contains(&"sort_order".to_string()));

        let group_orders: Vec<(String, i64)> = conn
            .prepare("SELECT name, sort_order FROM groups ORDER BY sort_order ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let note_orders: Vec<(String, i64)> = conn
            .prepare("SELECT name, sort_order FROM notes ORDER BY sort_order ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(group_orders, vec![("newer".into(), 0), ("older".into(), 1)]);
        assert_eq!(note_orders, vec![("second".into(), 0), ("first".into(), 1)]);
    }

    #[test]
    fn creates_expected_indexes() {
        let conn = Connection::open_in_memory().unwrap();
        initialize(&conn).unwrap();

        let indexes: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type = 'index'")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert!(indexes.contains(&"idx_groups_sort_order".to_string()));
        assert!(indexes.contains(&"idx_notes_sort_order".to_string()));
        assert!(indexes.contains(&"idx_notes_group_sort_order".to_string()));
    }
}
