use std::{path::PathBuf, process::Command};

pub mod date {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn today() -> Option<String> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|duration| {
                let days_since_epoch = duration.as_secs() / 86400;
                let (year, month, day) = calculate_date(days_since_epoch as u32);
                format!("{:02}/{:02}/{:04}", day, month, year)
            })
    }

    fn calculate_date(days: u32) -> (u32, u32, u32) {
        let mut year = 1970;
        let mut days_remaining = days;

        while days_remaining >= days_in_year(year) {
            days_remaining -= days_in_year(year);
            year += 1;
        }

        let mut month = 1;
        while let Some(days_in_month) = days_in_month(year, month) {
            if days_remaining < days_in_month {
                break;
            }
            days_remaining -= days_in_month;
            month += 1;
        }

        let day = days_remaining + 1;

        (year, month, day)
    }

    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn days_in_year(year: u32) -> u32 {
        if is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn days_in_month(year: u32, month: u32) -> Option<u32> {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
            4 | 6 | 9 | 11 => Some(30),
            2 => Some(if is_leap_year(year) { 29 } else { 28 }),
            _ => None,
        }
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


pub fn logo_bytes() -> String {
    rbase64::encode(super::LOGOBANNER)
}