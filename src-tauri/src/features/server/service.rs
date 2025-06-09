use rusqlite::{params, Connection};
use super::model::Server;
use ssh2::{Session, DisconnectCode};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;
use std::io::Read;
use once_cell::sync::Lazy;

static ACTIVE_SESSION: Lazy<Mutex<Option<Session>>> = Lazy::new(|| Mutex::new(None));

pub fn add_server(conn: &Connection, server: Server) -> Result<Server, String> {
    let now = chrono::Local::now().to_rfc3339();
    let created_at = server.created_at.unwrap_or(now.clone());
    let updated_at = server.updated_at.unwrap_or(now.clone());
    let settings_json = serde_json::to_string(&server.settings).unwrap_or_else(|_| "{}".to_string());

    conn.execute(
        "INSERT INTO servers (name, hostname, ip_address, port, username, ssh_key_id, notes, settings, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            server.name,
            server.hostname,
            server.ip_address,
            server.port,
            server.username,
            server.ssh_key_id,
            server.notes,
            settings_json,
            created_at,
            updated_at
        ],
    ).map_err(|e| e.to_string())?;

    // Assuming service::get_server is defined elsewhere or this refers to the get_server in this file
    self::get_server(&conn, conn.last_insert_rowid())
}

pub fn get_server(conn: &Connection, id: i64) -> Result<Server, String> {
    conn.query_row(
        "SELECT id, name, hostname, ip_address, port, username, ssh_key_id, notes, settings, created_at, updated_at
         FROM servers WHERE id = ?1",
        params![id],
        |row| {
            let settings_str: String = row.get(8)?;
            let settings = serde_json::from_str(&settings_str).unwrap_or_else(|_| serde_json::json!({}));

            Ok(Server {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                hostname: row.get(2)?,
                ip_address: row.get(3)?,
                port: row.get(4)?,
                username: row.get(5)?,
                ssh_key_id: row.get(6)?,
                notes: row.get(7)?,
                settings,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        }
    ).map_err(|e| e.to_string())
}

pub fn get_servers(conn: &Connection) -> Result<Vec<Server>, String> {
    let mut stmt = conn.prepare("
        SELECT id, name, hostname, ip_address, port, username, ssh_key_id, notes, settings, created_at, updated_at
        FROM servers
    ").map_err(|e| e.to_string())?;

    let server_iter = stmt.query_map([], |row| {
        let settings_str: String = row.get(8)?;
        let settings = serde_json::from_str(&settings_str).unwrap_or_else(|_| serde_json::json!({}));

        Ok(Server {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            hostname: row.get(2)?,
            ip_address: row.get(3)?,
            port: row.get(4)?,
            username: row.get(5)?,
            ssh_key_id: row.get(6)?,
            notes: row.get(7)?,
            settings,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut servers = Vec::new();
    for server in server_iter {
        servers.push(server.map_err(|e| e.to_string())?);
    }

    Ok(servers)
}

pub fn update_server(conn: &Connection, server: Server) -> Result<(), String> {
    let id = server.id.ok_or("Server ID is required for update")?;

    let now = chrono::Local::now().to_rfc3339();
    let updated_at = server.updated_at.unwrap_or(now.clone());

    conn.execute(
        "UPDATE servers SET
         name = ?1,
         hostname = ?2,
         ip_address = ?3,
         port = ?4,
         username = ?5,
         ssh_key_id = ?6,
         notes = ?7,
         updated_at = ?8
         WHERE id = ?9",
        params![
            server.name,
            server.hostname,
            server.ip_address,
            server.port,
            server.username,
            server.ssh_key_id,
            server.notes,
            updated_at,
            id
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn delete_server(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM servers WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn connect_with_ssh_key(server: &Server, ssh_key_path: &str) -> Result<Session, String> {
    let tcp = TcpStream::connect(format!("{}:{}", server.hostname, server.port))
        .map_err(|e| format!("Failed to connect to server: {}", e))?;

    let mut sess = Session::new()
        .map_err(|e| format!("Failed to create SSH session: {}", e))?;
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| format!("SSH handshake failed: {}", e))?;

    let path = Path::new(ssh_key_path);
    sess.userauth_pubkey_file(&server.username, None, path, None)
        .map_err(|e| format!("SSH key authentication failed: {}", e))?;

    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }

    Ok(sess)
}

pub fn connect_with_password(server: &Server, password: &str) -> Result<Session, String> {
    let tcp = TcpStream::connect(format!("{}:{}", server.hostname, server.port))
        .map_err(|e| format!("Failed to connect to server: {}", e))?;

    let mut sess = Session::new()
        .map_err(|e| format!("Failed to create SSH session: {}", e))?;
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| format!("SSH handshake failed: {}", e))?;

    sess.userauth_password(&server.username, password)
        .map_err(|e| format!("Password authentication failed: {}", e))?;

    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }
    
    Ok(sess)
}

pub fn get_ssh_key_path(conn: &Connection, ssh_key_id: i64) -> Result<String, String> {
    conn.query_row(
        "SELECT path FROM ssh_keys WHERE id = ?1",
        params![ssh_key_id],
        |row| row.get(0)
    ).map_err(|e| format!("Failed to get SSH key path: {}", e))
}

pub fn try_connect_to_server(conn: &Connection, server: &Server) -> Result<(), String> {
    let session_result = if let Some(ssh_key_id) = server.ssh_key_id {
        match get_ssh_key_path(conn, ssh_key_id) {
            Ok(ssh_key_path) => {
                let substr = ".pub";
                let private_key_path = ssh_key_path.replace(substr, "");
                connect_with_ssh_key(server, &private_key_path)
            },
            Err(e) => Err(format!("Failed to get SSH key: {}", e))
        }
    } else {
        Err("No SSH key set for this server, and password flow not initiated from here.".to_string())
    };

    match session_result {
        Ok(session) => {
            let mut active_session_guard = ACTIVE_SESSION.lock().map_err(|_| "Failed to acquire session lock for connect".to_string())?;
            *active_session_guard = Some(session);
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn disconnect_from_server() -> Result<(), String> {
    let mut active_session_guard = ACTIVE_SESSION.lock().map_err(|_| "Failed to acquire session lock for disconnect".to_string())?;

    if let Some(session) = active_session_guard.take() { 
        match session.disconnect(Some(DisconnectCode::ByApplication), "User initiated disconnect", Some("")) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to gracefully disconnect SSH session: {}. Session cleared.", e);
                Err(format!("SSH disconnect call failed: {}", e))
            }
        }
    } else {
        Err("No active session to disconnect.".to_string())
    }
}

pub fn cmd(command: &str) -> Result<String, String> {
    let mut active_session_guard = ACTIVE_SESSION.lock().map_err(|_| "Failed to acquire session lock for command execution".to_string())?;

    if let Some(session) = active_session_guard.as_mut() {
        if !session.authenticated() {
            return Err("Session is not authenticated.".to_string());
        }

        let mut channel = match session.channel_session() {
            Ok(ch) => ch,
            Err(e) => return Err(format!("Failed to open SSH channel: {}", e)),
        };

        if let Err(e) = channel.exec(command) {
            return Err(format!("Failed to execute command '{}': {}", command, e));
        }

        let mut output = String::new();
        if let Err(e) = channel.read_to_string(&mut output) {
            eprintln!("Warning: Failed to read command output: {}", e);
        }

        let mut stderr_output = String::new();
        if let Err(e) = channel.stderr().read_to_string(&mut stderr_output) {
            eprintln!("Warning: Failed to read command stderr: {}", e);
        }

        if !stderr_output.is_empty() {
            output.push_str("\n--- STDERR ---\n");
            output.push_str(&stderr_output);
        }

        match channel.wait_close() {
            Ok(_) => {},
            Err(e) => eprintln!("Warning: Error during channel close: {}", e),
        }

        let exit_status = match channel.exit_status() {
            Ok(status) => status,
            Err(e) => {
                return Err(format!("Failed to get command exit status: {}", e));
            }
        };

        if exit_status == 0 {
            Ok(output)
        } else {
            Err(format!("Command '{}' exited with status {}.\nOutput:\n{}\nStderr:\n{}", command, exit_status, output, stderr_output))
        }

    } else {
        Err("No active SSH session found.".to_string())
    }
}
