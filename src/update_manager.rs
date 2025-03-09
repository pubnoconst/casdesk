use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use reqwest::blocking::Client;
use serde::Deserialize;
use notify_rust::Notification;
use serde_json;
use log::{info, warn, error, debug};
use chrono::{Local, DateTime};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn update() {
    info!("[{}] Starting update check in background thread", timestamp());
    thread::spawn(|| {
        info!("[{}] Update check thread started", timestamp());
        if let Err(e) = check_and_update() {
            error!("[{}] Update check failed: {}", timestamp(), e);
            eprintln!("Update check failed: {}", e);
        }
    });
}

fn timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

fn check_and_update() -> Result<(), Box<dyn std::error::Error>> {
    info!("[{}] Beginning check for updates", timestamp());
    
    // Fetch current and latest versions
    let current_version = env!("CARGO_PKG_VERSION");
    info!("[{}] Current application version: {}", timestamp(), current_version);
    
    debug!("[{}] Creating HTTP client", timestamp());
    let client = Client::new();
    
    info!("[{}] Fetching latest release information from GitHub", timestamp());
    let resp = match client
        .get("https://api.github.com/repos/pubnoconst/casdesk/releases/latest")
        .header("User-Agent", "casdesk-app")
        .send() {
            Ok(response) => response,
            Err(e) => {
                error!("[{}] Failed to contact GitHub API: {}", timestamp(), e);
                return Err(e.into());
            }
        };
    
    if !resp.status().is_success() {
        let error_msg = format!("GitHub API error: {}", resp.status());
        error!("[{}] {}", timestamp(), error_msg);
        return Err(error_msg.into());
    }
    
    let resp_text = match resp.text() {
        Ok(text) => text,
        Err(e) => {
            error!("[{}] Failed to read GitHub API response: {}", timestamp(), e);
            return Err(e.into());
        }
    };
    
    debug!("[{}] Parsing JSON response from GitHub", timestamp());
    let release: Release = match serde_json::from_str(&resp_text) {
        Ok(release) => release,
        Err(e) => {
            error!("[{}] Failed to parse GitHub API response: {}", timestamp(), e);
            return Err(e.into());
        }
    };
    
    let latest_version = release.tag_name.trim_start_matches('v');
    info!("[{}] Latest available version: {}", timestamp(), latest_version);
    
    // Skip if already on latest version
    if current_version == latest_version {
        info!("[{}] Already on latest version, skipping update", timestamp());
        return Ok(());
    }
    
    info!("[{}] New version found: {} -> {}", timestamp(), current_version, latest_version);
    
    // OS-specific update handling
    #[cfg(target_os = "windows")]
    {
        info!("[{}] Handling update for Windows platform", timestamp());
        handle_windows_update(&release)?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        info!("[{}] Handling update for non-Windows platform", timestamp());
        handle_other_platform_update(&release)?;
    }
    
    info!("[{}] Update process completed successfully", timestamp());
    Ok(())
}

#[cfg(target_os = "windows")]
fn handle_windows_update(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    info!("[{}] Preparing Windows update process", timestamp());
    
    // Notify user that an update was found
    info!("[{}] Showing update notification to user", timestamp());
    match Notification::new()
        .appname("Casdesk")
        .summary("Update Available")
        .body("A new version has been found. It will be installed when you close the app.")
        .show() {
            Ok(_) => debug!("[{}] Successfully showed notification", timestamp()),
            Err(e) => warn!("[{}] Failed to show notification: {}", timestamp(), e)
        }
    
    // Find executable asset
    info!("[{}] Looking for executable in release assets", timestamp());
    let asset = match release.assets.iter().find(|asset| asset.name.ends_with(".exe")) {
        Some(a) => {
            info!("[{}] Found executable asset: {}", timestamp(), a.name);
            a
        },
        None => {
            let error_msg = "No executable found in release";
            error!("[{}] {}", timestamp(), error_msg);
            return Err(error_msg.into());
        }
    };
    
    // Get downloads directory
    debug!("[{}] Determining download directory", timestamp());
    let downloads_dir = if let Ok(home) = env::var("USERPROFILE") {
        let dir = PathBuf::from(home).join("Downloads");
        info!("[{}] Using user's Downloads directory: {}", timestamp(), dir.display());
        dir
    } else {
        let dir = env::temp_dir();
        info!("[{}] Using temp directory for download: {}", timestamp(), dir.display());
        dir
    };
    
    let download_path = downloads_dir.join("Casdesk.New.exe");
    info!("[{}] Update will be downloaded to: {}", timestamp(), download_path.display());
    
    // Download new version
    info!("[{}] Starting download from: {}", timestamp(), asset.browser_download_url);
    let mut response = match Client::new().get(&asset.browser_download_url).send() {
        Ok(resp) => resp,
        Err(e) => {
            error!("[{}] Failed to start download: {}", timestamp(), e);
            return Err(e.into());
        }
    };
    
    if !response.status().is_success() {
        let error_msg = format!("Download failed with status: {}", response.status());
        error!("[{}] {}", timestamp(), error_msg);
        return Err(error_msg.into());
    }
    
    info!("[{}] Creating destination file", timestamp());
    let mut file = match fs::File::create(&download_path) {
        Ok(file) => file,
        Err(e) => {
            error!("[{}] Failed to create download file: {}", timestamp(), e);
            return Err(e.into());
        }
    };
    
    info!("[{}] Copying download data to file", timestamp());
    match io::copy(&mut response, &mut file) {
        Ok(bytes) => info!("[{}] Downloaded {} bytes successfully", timestamp(), bytes),
        Err(e) => {
            error!("[{}] Failed to download file: {}", timestamp(), e);
            return Err(e.into());
        }
    }
    
    // Create completely silent updater
    info!("[{}] Creating updater scripts", timestamp());
    match create_completely_silent_updater(&download_path) {
        Ok(_) => info!("[{}] Updater scripts created successfully", timestamp()),
        Err(e) => {
            error!("[{}] Failed to create updater scripts: {}", timestamp(), e);
            return Err(e);
        }
    }
    
    info!("[{}] Windows update process prepared successfully", timestamp());
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_completely_silent_updater(new_exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("[{}] Setting up silent updater for Windows", timestamp());
    
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::iter::once;
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::SW_HIDE;
    
    debug!("[{}] Getting current executable path", timestamp());
    let current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            error!("[{}] Failed to get current executable path: {}", timestamp(), e);
            return Err(e.into());
        }
    };
    
    debug!("[{}] Getting executable filename", timestamp());
    let current_exe_name = match current_exe.file_name() {
        Some(name) => {
            let name_str = name.to_string_lossy();
            info!("[{}] Current executable name: {}", timestamp(), name_str);
            name_str
        },
        None => {
            let error_msg = "Failed to get executable name";
            error!("[{}] {}", timestamp(), error_msg);
            return Err(error_msg.into());
        }
    };
    
    // Create PowerShell script that runs completely hidden
    let ps_path = new_exe_path.with_file_name("update_casdesk.ps1");
    info!("[{}] Creating PowerShell updater script at: {}", timestamp(), ps_path.display());
    
    let ps_content = format!(
        r#"
# Keep checking until the app is closed
do {{
    $running = Get-Process -Name "{}" -ErrorAction SilentlyContinue
    if ($null -eq $running) {{ break }}
    Start-Sleep -Seconds 2
}} while ($true)

# Replace the executable
Copy-Item -Path "{}" -Destination "{}" -Force
Remove-Item -Path "{}" -Force
Remove-Item -Path $MyInvocation.MyCommand.Path -Force
"#,
        current_exe_name.trim_end_matches(".exe"),
        new_exe_path.display().to_string().replace("\\", "\\"),
        current_exe.display().to_string().replace("\\", "\\"),
        new_exe_path.display().to_string().replace("\\", "\\")
    );
    
    debug!("[{}] Writing PowerShell script content", timestamp());
    match fs::write(&ps_path, ps_content) {
        Ok(_) => info!("[{}] PowerShell script created successfully", timestamp()),
        Err(e) => {
            error!("[{}] Failed to write PowerShell script: {}", timestamp(), e);
            return Err(e.into());
        }
    }
    
    // Create VBS launcher that runs PowerShell with absolutely no window
    let vbs_path = new_exe_path.with_file_name("launch_updater.vbs");
    info!("[{}] Creating VBS launcher at: {}", timestamp(), vbs_path.display());
    let vbs_content = format!(
        r#"
command = "powershell.exe -WindowStyle Hidden -ExecutionPolicy Bypass -File ""{0}"""
CreateObject("WScript.Shell").Run command, 0, False
"#,
        ps_path.display().to_string().replace("\\", "\\\\")
    );
    
    debug!("[{}] Writing VBS launcher content", timestamp());
    match fs::write(&vbs_path, vbs_content) {
        Ok(_) => info!("[{}] VBS launcher created successfully", timestamp()),
        Err(e) => {
            error!("[{}] Failed to write VBS launcher: {}", timestamp(), e);
            return Err(e.into());
        }
    }
    
    // Use ShellExecuteW to launch the VBS file completely hidden
    info!("[{}] Preparing to launch updater script", timestamp());
    let vbs_path_wide: Vec<u16> = OsStr::new(vbs_path.as_os_str())
        .encode_wide()
        .chain(once(0))
        .collect();
    
    let operation: Vec<u16> = OsStr::new("open")
        .encode_wide()
        .chain(once(0))
        .collect();
    
    info!("[{}] Launching updater with ShellExecuteW", timestamp());
    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            vbs_path_wide.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_HIDE
        );
    }
    
    info!("[{}] Silent updater launched successfully", timestamp());
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn handle_other_platform_update(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    info!("[{}] Handling update for non-Windows platform", timestamp());
    
    let os_name = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "this platform"
    };
    info!("[{}] Detected OS: {}", timestamp(), os_name);
    
    // Notify user that automatic updates aren't supported on this OS
    info!("[{}] Showing platform-specific update notification", timestamp());
    match Notification::new()
        .appname("Casdesk")
        .summary("Update Available")
        .body(&format!(
            "A new version is available. Self-updating on {} isn't supported yet. \
            Please visit {} to download the latest version.",
            os_name, release.html_url
        ))
        .show() {
            Ok(_) => info!("[{}] Successfully showed notification", timestamp()),
            Err(e) => warn!("[{}] Failed to show notification: {}", timestamp(), e)
        };
    
    info!("[{}] Non-Windows update handling completed", timestamp());
    Ok(())
}
