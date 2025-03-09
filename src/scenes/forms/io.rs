use std::{path::PathBuf, process::Command};

pub mod date {
    use chrono::prelude::*;

    pub fn today() -> Option<String> {
        let today: DateTime<Local> = Local::now();
        Some(today.format("%d/%m/%Y").to_string())
    }
}

pub fn open(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("chrome").arg(path).spawn().or_else(|_| {
            Command::new("cmd").arg("/c").arg("start").arg(path).spawn()
        });
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg("-a").arg("Google Chrome").arg(path).spawn().or_else(|_| {
            Command::new("open").arg(path).spawn()
        });
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("google-chrome").arg(path).spawn().or_else(|_| {
            Command::new("xdg-open").arg(path).spawn()
        });
    }

    Ok(())
}
