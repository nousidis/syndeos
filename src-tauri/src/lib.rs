mod common;
mod features;
mod database;
use tauri::AppHandle;

#[tauri::command]
fn init_app(app_handle: AppHandle) -> Result<String, String> {
    let db_result = database::initialize(app_handle.clone())?;

    println!("Tauri SQLite Database Initialization Successful!");

    let _ = features::setting::init_default_settings(app_handle.clone())?;

    println!("Tauri Settings Initialization Successful!");

    let _ = features::ssh_key::init_ssh_keys(app_handle.clone())?;

    println!("Tauri SSH Keys Initialization Successful!");

    Ok(db_result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_app,

            features::server::add_server,
            features::server::get_server,
            features::server::update_server,
            features::server::delete_server,
            features::server::get_servers,
            features::server::try_connect_to_server,
            features::server::connect_with_password,
            features::server::disconnect_from_server,
            features::server::test,
            features::server::install_node,

            features::ssh_key::add_ssh_key,
            features::ssh_key::delete_ssh_key,
            features::ssh_key::get_ssh_key,
            features::ssh_key::get_ssh_keys,
            features::ssh_key::set_default_ssh_key,
            features::ssh_key::generate_ssh_key,

            features::setting::get_setting,
            features::setting::get_settings,
            features::setting::update_setting, 
            features::setting::reset_app, 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
