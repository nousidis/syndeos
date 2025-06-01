use rusqlite::{Connection};
use tauri::{AppHandle, Manager};

pub fn get(app_handle: &AppHandle) -> Result<Connection, String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("syndeos.db");
    Connection::open(&db_path).map_err(|e| e.to_string())
}
