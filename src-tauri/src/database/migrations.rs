use rusqlite::{Connection, Result as SqliteResult, Transaction};
use std::collections::HashMap;

const CURRENT_DB_VERSION: i32 = 1;

fn version_table_exists(tx: &Transaction) -> SqliteResult<bool> {
    let count: i32 = tx.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='db_version'",
        [],
        |row| row.get(0),
    )?;

    Ok(count > 0)
}

fn get_db_version(tx: &Transaction) -> SqliteResult<i32> {
    tx.query_row(
        "SELECT version FROM db_version WHERE id = 1",
        [],
        |row| row.get(0),
    )
}

fn update_db_version(tx: &Transaction, version: i32) -> SqliteResult<()> {
    tx.execute(
        "UPDATE db_version SET version = ?1 WHERE id = 1",
        [version],
    )?;

    Ok(())
}

fn migration_executed(tx: &Transaction, version: i32) -> SqliteResult<bool> {
    let current_version = get_db_version(tx).unwrap_or(0);
    
    Ok(current_version >= version)
}

type MigrationFn = fn(&Transaction) -> SqliteResult<()>;

fn migrate_to_v1(tx: &Transaction) -> SqliteResult<()> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS servers (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                hostname TEXT NOT NULL,
                ip_address TEXT NOT NULL,
                port INTEGER NOT NULL DEFAULT 22,
                username TEXT NOT NULL,
                ssh_key_id INTEGER,
                notes TEXT,
                settings TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (ssh_key_id) REFERENCES ssh_keys (id)
            )",
        [],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL
            )",
        [],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS ssh_keys (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                path TEXT NOT NULL,
                password TEXT,
                is_default BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
        [],
    )?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS db_version (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            version INTEGER NOT NULL
        )",
        [],
    )?;

    tx.execute(
        "INSERT INTO db_version (id, version) VALUES (1, ?1)",
        [1],
    )?;

    Ok(())
}

fn get_migrations() -> HashMap<i32, MigrationFn> {
    let mut migrations: HashMap<i32, MigrationFn> = HashMap::new();

    migrations.insert(1, migrate_to_v1);

    migrations
}

pub fn check_and_apply_migrations(tx: &mut Connection) -> Result<(), String> {
    let tx = tx.transaction().map_err(|e| e.to_string())?;

    let has_version_table = version_table_exists(&tx).unwrap_or(false);
    
    if !has_version_table {
        let migrations = get_migrations();
        if let Some(migration_fn) = migrations.get(&1) {
            migration_fn(&tx).map_err(|e| e.to_string())?;
            update_db_version(&tx, 1).map_err(|e| e.to_string())?;
        } else {
            tx.rollback().map_err(|e| e.to_string())?;
            return Err("Missing migration for version 1".to_string());
        }
    }

    let migrations = get_migrations();

    for version in 1..=CURRENT_DB_VERSION {
        if migration_executed(&tx, version).map_err(|e| e.to_string())? {
            continue;
        }

        if let Some(migration_fn) = migrations.get(&version) {
            migration_fn(&tx).map_err(|e| e.to_string())?;
            update_db_version(&tx, version).map_err(|e| e.to_string())?;
        } else {
            tx.rollback().map_err(|e| e.to_string())?;
            return Err(format!("Missing migration for version {}", version));
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}
