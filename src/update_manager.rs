use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use reqwest::blocking::Client;
use serde::Deserialize;
use notify_rust::Notification;
use serde_json;

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
    thread::spawn(|| {
        if let Err(e) = check_and_update() {
            eprintln!("Update check failed: {}", e);
        }
    });
}

fn check_and_update() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch current and latest versions
    let current_version = env!("CARGO_PKG_VERSION");
    
    let client = Client::new();
    let resp = client
        .get("https://api.github.com/repos/pubnoconst/casdesk/releases/latest")
        .header("User-Agent", "casdesk-app")
        .send()?;
    
    if !resp.status().is_success() {
        return Err(format!("GitHub API error: {}", resp.status()).into());
    }
    
    let release: Release = serde_json::from_str(&resp.text()?)?;
    let latest_version = release.tag_name.trim_start_matches('v');
    
    // Skip if already on latest version
    if current_version == latest_version {
        return Ok(());
    }
    
    // OS-specific update handling
    #[cfg(target_os = "windows")]
    {
        handle_windows_update(&release)?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        handle_other_platform_update(&release)?;
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn handle_windows_update(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    // Notify user that an update was found
    let _ = Notification::new()
        .appname("Casdesk")
        .summary("Update Available")
        .body("A new version has been found. It will be installed when you close the app.")
        .show();
    
    // Find executable asset
    let asset = match release.assets.iter().find(|asset| asset.name.ends_with(".exe")) {
        Some(a) => a,
        None => return Err("No executable found in release".into()),
    };
    
    // Get downloads directory
    let downloads_dir = if let Ok(home) = env::var("USERPROFILE") {
        PathBuf::from(home).join("Downloads")
    } else {
        env::temp_dir()
    };
    
    let download_path = downloads_dir.join("Casdesk.New.exe");
    
    // Download new version
    let mut response = Client::new().get(&asset.browser_download_url).send()?;
    if !response.status().is_success() {
        return Err("Download failed".into());
    }
    
    let mut file = fs::File::create(&download_path)?;
    io::copy(&mut response, &mut file)?;
    
    // Create completely silent updater
    create_completely_silent_updater(&download_path)?;
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_completely_silent_updater(new_exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::iter::once;
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::SW_HIDE;
    
    let current_exe = env::current_exe()?;
    let current_exe_name = current_exe.file_name()
        .ok_or("Failed to get executable name")?
        .to_string_lossy();
    
    // Create PowerShell script that runs completely hidden
    let ps_path = new_exe_path.with_file_name("update_casdesk.ps1");
    
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
    
    fs::write(&ps_path, ps_content)?;
    
    // Create VBS launcher that runs PowerShell with absolutely no window
    let vbs_path = new_exe_path.with_file_name("launch_updater.vbs");
    let vbs_content = format!(
        r#"
command = "powershell.exe -WindowStyle Hidden -ExecutionPolicy Bypass -File ""{0}"""
CreateObject("WScript.Shell").Run command, 0, False
"#,
        ps_path.display().to_string().replace("\\", "\\\\")
    );
    
    fs::write(&vbs_path, vbs_content)?;
    
    // Use ShellExecuteW to launch the VBS file completely hidden
    let vbs_path_wide: Vec<u16> = OsStr::new(vbs_path.as_os_str())
        .encode_wide()
        .chain(once(0))
        .collect();
    
    let operation: Vec<u16> = OsStr::new("open")
        .encode_wide()
        .chain(once(0))
        .collect();
    
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
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn handle_other_platform_update(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    let os_name = if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "this platform"
    };
    
    // Notify user that automatic updates aren't supported on this OS
    let _ = Notification::new()
        .appname("Casdesk")
        .summary("Update Available")
        .body(&format!(
            "A new version is available. Self-updating on {} isn't supported yet. \
            Please visit {} to download the latest version.",
            os_name, release.html_url
        ))
        .show();
    
    Ok(())
}
