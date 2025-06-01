use std::fs;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};
use super::migrations;

#[tauri::command]
pub fn initialize(app_handle: AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path().app_data_dir().unwrap();

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    let db_path = app_dir.join("syndeos.db");

    let mut conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    migrations::check_and_apply_migrations(&mut conn)?;

    Ok("Database initialized successfully".to_string())
}