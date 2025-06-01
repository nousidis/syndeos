use super::service;
use tauri::AppHandle;
use super::model::Server;
use crate::database::connection;

#[tauri::command]
pub fn get_server(app_handle: AppHandle, id: i64) -> Result<Server, String> {
    let conn = connection::get(&app_handle)?;

    service::get_server(&conn, id)
}

#[tauri::command]
pub fn get_servers(app_handle: AppHandle) -> Result<Vec<Server>, String> {
    let conn = connection::get(&app_handle)?;

    service::get_servers(&conn)
}

#[tauri::command]
pub fn add_server(app_handle: AppHandle, server: Server) -> Result<Server, String> {
    let conn = connection::get(&app_handle)?;

    service::add_server(&conn, server)
}

#[tauri::command]
pub fn update_server(app_handle: AppHandle, server: Server) -> Result<(), String> {
    let conn = connection::get(&app_handle)?;

    service::update_server(&conn, server)
}

#[tauri::command]
pub fn delete_server(app_handle: AppHandle, id: i64) -> Result<(), String> {
    let conn = connection::get(&app_handle)?;

    service::delete_server(&conn, id)
}

#[tauri::command]
pub fn try_connect_to_server(app_handle: AppHandle, id: i64) -> Result<bool, String> {
    let conn = connection::get(&app_handle)?;

    let server = service::get_server(&conn, id)?;

    match service::try_connect_to_server(&conn, &server) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

#[tauri::command]
pub fn connect_with_password(app_handle: AppHandle, id: i64, password: String) -> Result<bool, String> {
    let conn = connection::get(&app_handle)?;

    let server = service::get_server(&conn, id)?;

    match service::connect_with_password(&server, &password) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

#[tauri::command]
pub fn disconnect_from_server() -> Result<bool, String> {
    match service::disconnect_from_server() {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

#[tauri::command]
pub fn run_cmd() -> Result<String, String> {
    service::run_cmd("ls")
}
