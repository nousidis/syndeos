use std;
use std::fs;
use rusqlite::params;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};
use super::model::Setting;

pub fn init_default_settings(conn: Connection) -> Result<(), String> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM settings")
        .map_err(|e| e.to_string())?;
    let count: i64 = stmt.query_row([], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if count == 0 {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)",
            params!["ui/theme", "system"],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)",
            params!["ui/default_view", "servers"],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)",
            params!["connection/ssh_timeout", "30"],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2, ?3)",
            params!["server/default_port", "22"],
        ).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_setting(conn: Connection, key: String) -> Result<String, String> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0)
    ).map_err(|e| e.to_string())
}

pub fn get_settings(conn: Connection) -> Result<Vec<Setting>, String> {
    let mut stmt = conn.prepare("SELECT id, key, value FROM settings")
        .map_err(|e| e.to_string())?;

    let settings_iter = stmt.query_map([], |row| {
        Ok(Setting {
            id: Some(row.get(0)?),
            key: row.get(1)?,
            value: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut settings = Vec::new();
    for setting in settings_iter {
        settings.push(setting.map_err(|e| e.to_string())?);
    }

    Ok(settings)
}

pub fn update_setting(conn: Connection, key: String, value: String) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn reset_app(app_handle: AppHandle) -> Result<(), String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("syndeos.db");

    // Check if the file exists
    if db_path.exists() {
        // Delete the database file
        fs::remove_file(&db_path).map_err(|e| format!("Failed to delete database: {}", e))?;
    }

    // Schedule a restart of the application after a short delay
    // This gives the frontend time to receive the response
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        app_handle.restart();
    });

    Ok(())
}