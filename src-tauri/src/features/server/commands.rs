use super::service;
use tauri::AppHandle;
use super::model::Server;
use crate::database::connection;

// =============================================================================
// PHP VERSION MANAGEMENT COMMANDS
// =============================================================================

/// Install a specific PHP version with common extensions
#[tauri::command]
pub fn install_php_version(version: String) -> Result<String, String> {
    // Check if EPEL and Remi repositories are installed
    let epel_check = service::cmd("dnf repolist | grep epel || echo 'not found'");
    let remi_check = service::cmd("dnf repolist | grep remi || echo 'not found'");

    // Install EPEL repository if not present
    if epel_check.is_err() || epel_check.unwrap().trim() == "not found" {
        service::cmd("sudo dnf install -y epel-release")
            .map_err(|e| format!("Failed to install EPEL repository: {}", e))?;
    }

    // Install Remi repository if not present
    if remi_check.is_err() || remi_check.unwrap().trim() == "not found" {
        service::cmd("sudo dnf install -y https://rpms.remirepo.net/enterprise/remi-release-9.rpm")
            .map_err(|e| format!("Failed to install Remi repository: {}", e))?;
    }

    // Enable the specific PHP version module
    service::cmd(&format!("sudo dnf module reset php -y"))
        .map_err(|e| format!("Failed to reset PHP module: {}", e))?;

    service::cmd(&format!("sudo dnf module enable php:remi-{} -y", version))
        .map_err(|e| format!("Failed to enable PHP {} module: {}", version, e))?;

    // Install PHP and common extensions
    let extensions = vec![
        "php", "php-fpm", "php-common", "php-mysqlnd", "php-xml", "php-curl",
        "php-gd", "php-imagick", "php-cli", "php-devel", "php-imap",
        "php-mbstring", "php-opcache", "php-soap", "php-zip", "php-intl"
    ];

    let install_cmd = format!("sudo dnf install -y {}", extensions.join(" "));
    service::cmd(&install_cmd)
        .map_err(|e| format!("Failed to install PHP {}: {}", version, e))?;

    // Verify installation
    service::cmd("php -v")
        .map_err(|e| format!("PHP {} installation verification failed: {}", version, e))
}

/// Remove a specific PHP version and its extensions
#[tauri::command]
pub fn remove_php_version(version: String) -> Result<String, String> {
    // List installed PHP packages
    let list_cmd = "rpm -qa | grep php";
    let packages_output = service::cmd(list_cmd)
        .map_err(|e| format!("Failed to list PHP packages: {}", e))?;

    if packages_output.trim().is_empty() {
        return Err(format!("No PHP packages are installed"));
    }

    // Remove all PHP packages
    service::cmd("sudo dnf remove -y php php-*")
        .map_err(|e| format!("Failed to remove PHP packages: {}", e))?;

    // Reset PHP module to allow installation of different version
    service::cmd("sudo dnf module reset php -y")
        .map_err(|e| format!("Failed to reset PHP module: {}", e))?;

    Ok(format!("PHP {} successfully removed", version))
}

/// List all installed PHP versions
#[tauri::command]
pub fn list_php_versions() -> Result<Vec<String>, String> {
    // Check available PHP modules
    let output = service::cmd("dnf module list php | grep php | awk '{print $2}' | grep -E '^remi-[0-9]+\\.[0-9]+$' | sed 's/remi-//'")
        .map_err(|e| format!("Failed to list PHP versions: {}", e))?;

    let versions: Vec<String> = output
        .trim()
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect();

    Ok(versions)
}

/// Set the default PHP version system-wide
#[tauri::command]
pub fn set_default_php_version(version: String) -> Result<String, String> {
    // Reset current PHP module
    service::cmd("sudo dnf module reset php -y")
        .map_err(|e| format!("Failed to reset PHP module: {}", e))?;

    // Enable the specified PHP version module
    service::cmd(&format!("sudo dnf module enable php:remi-{} -y", version))
        .map_err(|e| format!("Failed to enable PHP {} module: {}", version, e))?;

    // Install/update PHP to the new version
    service::cmd("sudo dnf install -y php")
        .map_err(|e| format!("Failed to install PHP {}: {}", version, e))?;

    // Verify the change
    let verify_output = service::cmd("php -v")
        .map_err(|e| format!("Failed to verify PHP version change: {}", e))?;

    Ok(format!("PHP {} set as default. Current version: {}", version, verify_output.lines().next().unwrap_or("")))
}

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

// =============================================================================
// NODE.JS VERSION MANAGEMENT COMMANDS
// =============================================================================

/// Install a specific Node.js version via NVM
#[tauri::command]
pub fn install_node_version(version: String) -> Result<String, String> {
    // Check if NVM is installed
    let nvm_check = service::cmd("command -v nvm || echo 'not found'");

    if nvm_check.is_err() || nvm_check.unwrap().trim() == "not found" {
        // Install NVM
        service::cmd("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash")
            .map_err(|e| format!("Failed to install NVM: {}", e))?;

        // Source NVM for current session
        service::cmd("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\"")
            .map_err(|e| format!("Failed to source NVM: {}", e))?;
    }

    // Install the specified Node.js version
    let install_cmd = format!("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm install {}", version);
    service::cmd(&install_cmd)
        .map_err(|e| format!("Failed to install Node.js {}: {}", version, e))?;

    // Verify installation
    let verify_cmd = format!("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm use {} && node --version", version);
    service::cmd(&verify_cmd)
        .map_err(|e| format!("Node.js {} installation verification failed: {}", version, e))
}

/// Remove a specific Node.js version
#[tauri::command]
pub fn remove_node_version(version: String) -> Result<String, String> {
    // Check if NVM is installed
    let nvm_check = service::cmd("command -v nvm || echo 'not found'");
    if nvm_check.is_err() || nvm_check.unwrap().trim() == "not found" {
        return Err("NVM is not installed".to_string());
    }

    // Remove the specified Node.js version
    let remove_cmd = format!("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm uninstall {}", version);
    service::cmd(&remove_cmd)
        .map_err(|e| format!("Failed to remove Node.js {}: {}", version, e))?;

    Ok(format!("Node.js {} successfully removed", version))
}

/// List all installed Node.js versions
#[tauri::command]
pub fn list_node_versions() -> Result<Vec<String>, String> {
    // Check if NVM is installed
    let nvm_check = service::cmd("command -v nvm || echo 'not found'");
    if nvm_check.is_err() || nvm_check.unwrap().trim() == "not found" {
        return Ok(vec![]);
    }

    let output = service::cmd("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm list --no-colors")
        .map_err(|e| format!("Failed to list Node.js versions: {}", e))?;

    let versions: Vec<String> = output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("v") && !trimmed.contains("->") {
                Some(trimmed.trim_start_matches("v").to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(versions)
}

/// Set the default Node.js version
#[tauri::command]
pub fn set_default_node_version(version: String) -> Result<String, String> {
    // Check if NVM is installed
    let nvm_check = service::cmd("command -v nvm || echo 'not found'");
    if nvm_check.is_err() || nvm_check.unwrap().trim() == "not found" {
        return Err("NVM is not installed".to_string());
    }

    // Set the default version
    let set_cmd = format!("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm alias default {}", version);
    service::cmd(&set_cmd)
        .map_err(|e| format!("Failed to set Node.js {} as default: {}", version, e))?;

    // Verify the change
    let verify_cmd = "export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && node --version";
    let verify_output = service::cmd(verify_cmd)
        .map_err(|e| format!("Failed to verify Node.js version change: {}", e))?;

    Ok(format!("Node.js {} set as default. Current version: {}", version, verify_output.trim()))
}

// =============================================================================
// APPLICATION MANAGEMENT COMMANDS
// =============================================================================

/// Create a new application with dedicated user and runtime versions
#[tauri::command]
pub fn create_application(app_name: String, username: String, php_version: Option<String>, node_version: Option<String>) -> Result<String, String> {
    let php_ver = php_version.unwrap_or_else(|| "8.4".to_string());
    let node_ver = node_version.unwrap_or_else(|| "lts".to_string());

    // Check if user exists, create if not
    let user_exists = service::cmd(&format!("id -u {} &>/dev/null && echo 'exists' || echo 'not exists'", username));
    if user_exists.is_err() || user_exists.unwrap().trim() != "exists" {
        // Create the user with home directory
        service::cmd(&format!("sudo useradd -m -s /bin/bash {}", username))
            .map_err(|e| format!("Failed to create user {}: {}", username, e))?;

        // Add user to nginx group for web permissions
        service::cmd(&format!("sudo usermod -aG nginx {}", username))
            .map_err(|e| format!("Failed to add user to nginx group: {}", e))?;
    }

    // Create application directory in user's home
    let app_root = format!("/home/{}/app", username);
    service::cmd(&format!("sudo mkdir -p {}", app_root))
        .map_err(|e| format!("Failed to create application directory: {}", e))?;

    // Set proper ownership for application directory
    service::cmd(&format!("sudo chown -R {}:nginx {}", username, app_root))
        .map_err(|e| format!("Failed to set application directory ownership: {}", e))?;

    // Create application-specific log directory
    let log_dir = format!("/var/log/nginx/{}", app_name);
    service::cmd(&format!("sudo mkdir -p {}", log_dir))
        .map_err(|e| format!("Failed to create log directory: {}", e))?;

    service::cmd(&format!("sudo chown -R {}:nginx {}", username, log_dir))
        .map_err(|e| format!("Failed to set log directory ownership: {}", e))?;

    // Create a basic index.html file
    let index_content = format!("<html><head><title>{}</title></head><body><h1>Welcome to {}</h1><p>Your application has been successfully created!</p><p>User: {}</p><p>PHP Version: {}</p><p>Node Version: {}</p></body></html>",
        app_name, app_name, username, php_ver, node_ver);
    service::cmd(&format!("echo '{}' | sudo tee {}/index.html", index_content, app_root))
        .map_err(|e| format!("Failed to create index.html: {}", e))?;

    // Install NVM for the user if not already installed
    let nvm_check = service::cmd(&format!("sudo -u {} bash -c 'command -v nvm || echo \"not found\"'", username));
    if nvm_check.is_err() || nvm_check.unwrap().trim() == "not found" {
        // Try to install NVM via DNF first (Alma Linux package)
        let dnf_nvm = service::cmd("sudo dnf install -y nvm");
        if dnf_nvm.is_err() {
            // Fallback to curl installation if DNF package not available
            service::cmd(&format!("sudo -u {} bash -c 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash'", username))
                .map_err(|e| format!("Failed to install NVM for user {}: {}", username, e))?;
        }
    }

    // Install specified Node.js version for the user
    let node_install_cmd = format!("sudo -u {} bash -c 'export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm install {} && nvm use {}'",
        username, node_ver, node_ver);
    service::cmd(&node_install_cmd)
        .map_err(|e| format!("Failed to install Node.js {} for user {}: {}", node_ver, username, e))?;

    // Create PHP-FPM pool configuration for the application
    let pool_config = format!(r#"[{}]
user = {}
group = nginx
listen = /run/php-fpm/{}.sock
listen.owner = {}
listen.group = nginx
listen.mode = 0660
pm = dynamic
pm.max_children = 5
pm.start_servers = 2
pm.min_spare_servers = 1
pm.max_spare_servers = 3
php_admin_value[error_log] = /var/log/nginx/{}/php_errors.log
php_admin_flag[log_errors] = on
"#, app_name, username, app_name, username, app_name);

    service::cmd(&format!("echo '{}' | sudo tee /etc/php-fpm.d/{}.conf", pool_config, app_name))
        .map_err(|e| format!("Failed to create PHP-FPM pool configuration: {}", e))?;

    // Create Nginx configuration
    let nginx_config = format!(r#"server {{
    listen 80;
    listen [::]:80;

    root {};
    index index.php index.html index.htm;

    server_name {};

    access_log /var/log/nginx/{}/access.log;
    error_log /var/log/nginx/{}/error.log;

    location / {{
        try_files $uri $uri/ /index.php$is_args$args;
    }}

    location ~ \.php$ {{
        fastcgi_pass unix:/run/php-fpm/{}.sock;
        fastcgi_index index.php;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        include fastcgi_params;
    }}

    location ~ /\.ht {{
        deny all;
    }}
}}"#, app_root, app_name, app_name, app_name, app_name);

    // Write Nginx configuration file
    service::cmd(&format!("echo '{}' | sudo tee /etc/nginx/conf.d/{}.conf", nginx_config, app_name))
        .map_err(|e| format!("Failed to create Nginx configuration: {}", e))?;

    // Test Nginx configuration
    let test_result = service::cmd("sudo nginx -t");
    if test_result.is_err() {
        // If the test fails, remove the configuration to prevent Nginx from failing to start
        let _ = service::cmd(&format!("sudo rm -f /etc/nginx/conf.d/{}.conf", app_name));
        let _ = service::cmd(&format!("sudo rm -f /etc/php-fpm.d/{}.conf", app_name));
        return Err(format!("Nginx configuration test failed: {}", test_result.err().unwrap()));
    }

    // Reload PHP-FPM to load the new pool
    service::cmd("sudo systemctl reload php-fpm")
        .map_err(|e| format!("Failed to reload PHP-FPM: {}", e))?;

    // Reload Nginx
    service::cmd("sudo systemctl reload nginx")
        .map_err(|e| format!("Failed to reload Nginx: {}", e))?;

    Ok(format!("Application {} successfully created for user {} at {} (PHP: {}, Node: {})",
        app_name, username, app_root, php_ver, node_ver))
}

/// Remove an existing application
#[tauri::command]
pub fn remove_application(app_name: String) -> Result<String, String> {
    // Remove Nginx configuration
    service::cmd(&format!("sudo rm -f /etc/nginx/conf.d/{}.conf", app_name))
        .map_err(|e| format!("Failed to remove Nginx configuration: {}", e))?;

    // Remove PHP-FPM pool configuration
    service::cmd(&format!("sudo rm -f /etc/php-fpm.d/{}.conf", app_name))
        .map_err(|e| format!("Failed to remove PHP-FPM pool configuration: {}", e))?;

    // Remove application-specific log directory
    service::cmd(&format!("sudo rm -rf /var/log/nginx/{}", app_name))
        .map_err(|e| format!("Failed to remove log directory: {}", e))?;

    // Test Nginx configuration
    service::cmd("sudo nginx -t")
        .map_err(|e| format!("Nginx configuration test failed after removal: {}", e))?;

    // Reload PHP-FPM
    service::cmd("sudo systemctl reload php-fpm")
        .map_err(|e| format!("Failed to reload PHP-FPM: {}", e))?;

    // Reload Nginx
    service::cmd("sudo systemctl reload nginx")
        .map_err(|e| format!("Failed to reload Nginx: {}", e))?;

    // Note: Application directory and user are preserved for safety
    // They can be manually removed if needed

    Ok(format!("Application {} successfully removed", app_name))
}

/// List all configured applications with details
#[tauri::command]
pub fn list_applications() -> Result<Vec<serde_json::Value>, String> {
    let output = service::cmd("ls /etc/nginx/conf.d/*.conf 2>/dev/null | xargs -I {} basename {} .conf || echo ''")
        .map_err(|e| format!("Failed to list applications: {}", e))?;

    let app_names: Vec<&str> = output
        .trim()
        .split('\n')
        .filter(|w| !w.is_empty())
        .collect();

    let mut applications = Vec::new();

    for app_name in app_names {
        // Extract server_name from Nginx config
        let server_name_cmd = format!("grep 'server_name' /etc/nginx/conf.d/{}.conf | awk '{{print $2}}' | sed 's/;//'", app_name);
        let server_name = service::cmd(&server_name_cmd).unwrap_or_else(|_| app_name.to_string());

        // Extract user from PHP-FPM pool config
        let user_cmd = format!("grep '^user = ' /etc/php-fpm.d/{}.conf | awk '{{print $3}}' || echo 'unknown'", app_name);
        let user = service::cmd(&user_cmd).unwrap_or_else(|_| "unknown".to_string());

        // Get application root path
        let root_cmd = format!("grep 'root' /etc/nginx/conf.d/{}.conf | awk '{{print $2}}' | sed 's/;//'", app_name);
        let app_root = service::cmd(&root_cmd).unwrap_or_else(|_| format!("/home/{}/app", user.trim()));

        // Check if application is enabled (config file exists)
        let enabled = std::path::Path::new(&format!("/etc/nginx/conf.d/{}.conf", app_name)).exists();

        let app_info = serde_json::json!({
            "name": app_name,
            "server_name": server_name.trim(),
            "user": user.trim(),
            "root_path": app_root.trim(),
            "enabled": enabled,
            "log_directory": format!("/var/log/nginx/{}", app_name),
            "php_pool": format!("/etc/php-fpm.d/{}.conf", app_name),
            "nginx_config": format!("/etc/nginx/conf.d/{}.conf", app_name)
        });

        applications.push(app_info);
    }

    Ok(applications)
}

/// Enable an application
#[tauri::command]
pub fn enable_application(app_name: String) -> Result<String, String> {
    // Check if application configuration exists
    let nginx_config_path = format!("/etc/nginx/conf.d/{}.conf", app_name);
    let php_pool_path = format!("/etc/php-fpm.d/{}.conf", app_name);

    // Check if configurations exist
    service::cmd(&format!("test -f {}", nginx_config_path))
        .map_err(|_| format!("Application Nginx configuration for {} does not exist", app_name))?;

    service::cmd(&format!("test -f {}", php_pool_path))
        .map_err(|_| format!("Application PHP-FPM pool for {} does not exist", app_name))?;

    // Test Nginx configuration
    service::cmd("sudo nginx -t")
        .map_err(|e| format!("Nginx configuration test failed: {}", e))?;

    // Reload PHP-FPM to ensure pool is active
    service::cmd("sudo systemctl reload php-fpm")
        .map_err(|e| format!("Failed to reload PHP-FPM: {}", e))?;

    // Reload Nginx
    service::cmd("sudo systemctl reload nginx")
        .map_err(|e| format!("Failed to reload Nginx: {}", e))?;

    Ok(format!("Application {} successfully enabled", app_name))
}

/// Disable an application
#[tauri::command]
pub fn disable_application(app_name: String) -> Result<String, String> {
    // Move Nginx configuration to disabled state (rename with .disabled extension)
    let nginx_config = format!("/etc/nginx/conf.d/{}.conf", app_name);
    let nginx_disabled = format!("/etc/nginx/conf.d/{}.conf.disabled", app_name);

    service::cmd(&format!("sudo mv {} {} 2>/dev/null || true", nginx_config, nginx_disabled))
        .map_err(|e| format!("Failed to disable Nginx configuration for {}: {}", app_name, e))?;

    // Move PHP-FPM pool configuration to disabled state
    let php_pool = format!("/etc/php-fpm.d/{}.conf", app_name);
    let php_disabled = format!("/etc/php-fpm.d/{}.conf.disabled", app_name);

    service::cmd(&format!("sudo mv {} {} 2>/dev/null || true", php_pool, php_disabled))
        .map_err(|e| format!("Failed to disable PHP-FPM pool for {}: {}", app_name, e))?;

    // Test Nginx configuration
    service::cmd("sudo nginx -t")
        .map_err(|e| format!("Nginx configuration test failed: {}", e))?;

    // Reload PHP-FPM
    service::cmd("sudo systemctl reload php-fpm")
        .map_err(|e| format!("Failed to reload PHP-FPM: {}", e))?;

    // Reload Nginx
    service::cmd("sudo systemctl reload nginx")
        .map_err(|e| format!("Failed to reload Nginx: {}", e))?;

    Ok(format!("Application {} successfully disabled", app_name))
}

// =============================================================================
// USER MANAGEMENT COMMANDS
// =============================================================================

/// Create a new system user
#[tauri::command]
pub fn create_user(username: String, password: String, sudo_access: bool) -> Result<String, String> {
    // Check if user already exists
    let user_exists = service::cmd(&format!("id -u {} &>/dev/null && echo 'exists' || echo 'not exists'", username));

    if user_exists.is_ok() && user_exists.unwrap().trim() == "exists" {
        return Err(format!("User {} already exists", username));
    }

    // Create the user with home directory
    service::cmd(&format!("sudo useradd -m -s /bin/bash {}", username))
        .map_err(|e| format!("Failed to create user {}: {}", username, e))?;

    // Set the user password
    service::cmd(&format!("echo '{}:{}' | sudo chpasswd", username, password))
        .map_err(|e| format!("Failed to set password for user {}: {}", username, e))?;

    // Add user to nginx group for web permissions
    service::cmd(&format!("sudo usermod -aG nginx {}", username))
        .map_err(|e| format!("Failed to add user to nginx group: {}", e))?;

    // Add sudo access if requested
    if sudo_access {
        service::cmd(&format!("sudo usermod -aG wheel {}", username))
            .map_err(|e| format!("Failed to add user to wheel group: {}", e))?;
    }

    Ok(format!("User {} successfully created{}", username, if sudo_access { " with sudo access" } else { "" }))
}

/// Remove an existing user
#[tauri::command]
pub fn remove_user(username: String) -> Result<String, String> {
    // Check if user exists
    let user_exists = service::cmd(&format!("id -u {} &>/dev/null && echo 'exists' || echo 'not exists'", username));

    if user_exists.is_err() || user_exists.unwrap().trim() != "exists" {
        return Err(format!("User {} does not exist", username));
    }

    // Remove user's applications if they exist
    let apps_result = list_applications();
    if let Ok(apps) = apps_result {
        for app in apps {
            if let Some(app_user) = app.get("user") {
                if app_user.as_str() == Some(&username) {
                    if let Some(app_name) = app.get("name") {
                        let _ = remove_application(app_name.as_str().unwrap_or("").to_string());
                    }
                }
            }
        }
    }

    // Delete the user with home directory
    service::cmd(&format!("sudo userdel -r {}", username))
        .map_err(|e| format!("Failed to delete user {}: {}", username, e))?;

    Ok(format!("User {} successfully removed", username))
}

/// List all system users
#[tauri::command]
pub fn list_users() -> Result<Vec<String>, String> {
    // Get users with UID >= 1000 (regular users, not system users)
    let output = service::cmd("awk -F: '$3 >= 1000 && $3 != 65534 {print $1}' /etc/passwd")
        .map_err(|e| format!("Failed to list users: {}", e))?;

    let users: Vec<String> = output
        .trim()
        .split('\n')
        .filter(|u| !u.is_empty())
        .map(|u| u.to_string())
        .collect();

    Ok(users)
}

/// Change user password
#[tauri::command]
pub fn change_user_password(username: String, new_password: String) -> Result<String, String> {
    // Check if user exists
    let user_exists = service::cmd(&format!("id -u {} &>/dev/null && echo 'exists' || echo 'not exists'", username));

    if user_exists.is_err() || user_exists.unwrap().trim() != "exists" {
        return Err(format!("User {} does not exist", username));
    }

    // Change the user password
    service::cmd(&format!("echo '{}:{}' | sudo chpasswd", username, new_password))
        .map_err(|e| format!("Failed to change password for user {}: {}", username, e))?;

    Ok(format!("Password for user {} successfully changed", username))
}

// =============================================================================
// SERVER INITIAL SETUP COMMAND
// =============================================================================

/// Comprehensive server setup command that installs and configures all necessary components
#[tauri::command]
pub fn setup_server() -> Result<String, String> {
    let mut setup_log = Vec::new();

    // Update system packages
    setup_log.push("Updating system packages...".to_string());
    service::cmd("sudo dnf update -y")
        .map_err(|e| format!("Failed to update system packages: {}", e))?;

    // Install essential packages
    setup_log.push("Installing essential packages...".to_string());
    service::cmd("sudo dnf install -y curl wget git unzip tar gzip epel-release")
        .map_err(|e| format!("Failed to install essential packages: {}", e))?;

    // Install and configure Nginx
    setup_log.push("Installing and configuring Nginx...".to_string());
    service::cmd("sudo dnf install -y nginx")
        .map_err(|e| format!("Failed to install Nginx: {}", e))?;

    // Start and enable Nginx
    service::cmd("sudo systemctl start nginx")
        .map_err(|e| format!("Failed to start Nginx: {}", e))?;
    service::cmd("sudo systemctl enable nginx")
        .map_err(|e| format!("Failed to enable Nginx: {}", e))?;

    // Install and configure MariaDB
    setup_log.push("Installing and configuring MariaDB...".to_string());
    service::cmd("sudo dnf install -y mariadb-server mariadb")
        .map_err(|e| format!("Failed to install MariaDB: {}", e))?;

    // Start and enable MariaDB
    service::cmd("sudo systemctl start mariadb")
        .map_err(|e| format!("Failed to start MariaDB: {}", e))?;
    service::cmd("sudo systemctl enable mariadb")
        .map_err(|e| format!("Failed to enable MariaDB: {}", e))?;

    // Secure MariaDB installation (basic setup)
    service::cmd("sudo mysql -e \"UPDATE mysql.user SET Password = PASSWORD('root') WHERE User = 'root'; DELETE FROM mysql.user WHERE User=''; DELETE FROM mysql.user WHERE User='root' AND Host NOT IN ('localhost', '127.0.0.1', '::1'); DROP DATABASE IF EXISTS test; DELETE FROM mysql.db WHERE Db='test' OR Db='test\\_%'; FLUSH PRIVILEGES;\"")
        .map_err(|e| format!("Failed to secure MariaDB: {}", e))?;

    // Install NVM (Node Version Manager)
    setup_log.push("Installing NVM (Node Version Manager)...".to_string());
    service::cmd("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash")
        .map_err(|e| format!("Failed to install NVM: {}", e))?;

    // Install latest stable PHP version
    setup_log.push("Installing latest stable PHP version...".to_string());
    install_php_version("8.4".to_string())?;

    // Configure PHP-FPM
    service::cmd("sudo systemctl start php-fpm")
        .map_err(|e| format!("Failed to start PHP-FPM: {}", e))?;
    service::cmd("sudo systemctl enable php-fpm")
        .map_err(|e| format!("Failed to enable PHP-FPM: {}", e))?;

    // Set up basic security configurations
    setup_log.push("Setting up basic security configurations...".to_string());

    // Configure firewalld (RHEL/Alma Linux default firewall)
    service::cmd("sudo systemctl start firewalld")
        .map_err(|e| format!("Failed to start firewalld: {}", e))?;
    service::cmd("sudo systemctl enable firewalld")
        .map_err(|e| format!("Failed to enable firewalld: {}", e))?;
    service::cmd("sudo firewall-cmd --permanent --add-service=ssh")
        .map_err(|e| format!("Failed to allow SSH in firewalld: {}", e))?;
    service::cmd("sudo firewall-cmd --permanent --add-service=http")
        .map_err(|e| format!("Failed to allow HTTP in firewalld: {}", e))?;
    service::cmd("sudo firewall-cmd --permanent --add-service=https")
        .map_err(|e| format!("Failed to allow HTTPS in firewalld: {}", e))?;
    service::cmd("sudo firewall-cmd --reload")
        .map_err(|e| format!("Failed to reload firewalld: {}", e))?;

    // Create necessary directories and set permissions
    setup_log.push("Creating necessary directories...".to_string());
    service::cmd("sudo mkdir -p /var/www")
        .map_err(|e| format!("Failed to create /var/www directory: {}", e))?;
    service::cmd("sudo chown -R nginx:nginx /var/www")
        .map_err(|e| format!("Failed to set permissions on /var/www: {}", e))?;

    // Create a default index page
    let default_content = "<html><head><title>Server Setup Complete</title></head><body><h1>Welcome!</h1><p>Your server has been successfully configured with Nginx, MariaDB, PHP, and NVM.</p></body></html>";
    service::cmd(&format!("echo '{}' | sudo tee /usr/share/nginx/html/index.html", default_content))
        .map_err(|e| format!("Failed to create default index page: {}", e))?;

    // Configure SELinux for web services (Alma Linux specific)
    setup_log.push("Configuring SELinux for web services...".to_string());
    service::cmd("sudo setsebool -P httpd_can_network_connect 1")
        .map_err(|e| format!("Failed to configure SELinux for HTTP network connections: {}", e))?;
    service::cmd("sudo setsebool -P httpd_execmem 1")
        .map_err(|e| format!("Failed to configure SELinux for HTTP memory execution: {}", e))?;

    setup_log.push("Server setup completed successfully!".to_string());

    Ok(setup_log.join("\n"))
}

// =============================================================================
// TEST COMMAND
// =============================================================================

/// Test command for development purposes
#[tauri::command]
pub fn test() -> Result<String, String> {
    list_php_versions()
        .map(|versions| format!("Available PHP versions: {:?}", versions))
}
