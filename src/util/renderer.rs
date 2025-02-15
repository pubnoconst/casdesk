use dioxus::html::template;
use notify_rust::Notification;

use super::*;
use std::path::PathBuf;
use std::process::Command;

fn open_html_file(file_path: PathBuf) -> Result<(), std::io::Error> {
    let file_url = format!("file://{}", file_path.display());

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&[
                "/C",
                "start",
                "msedge",
                "--no-first-run",
                "--disable-extensions",
                "--disable-plugins",
                "--disable-sync",
                "--no-default-browser-check",
                "--no-service-autorun",
                "--noerrdialogs",
                "--start-maximized",
                &file_url,
            ])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(&[
                "-a",
                "Safari",
                "--args",
                "--no-default-browser-check",
                "--disable-extensions",
                "--disable-plugins",
                "--disable-sync",
                &file_url,
            ])
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(&file_url).spawn()?;
    }

    Ok(())
}

pub fn sales_form(customer: Customer, device: SellableDevice, date: String, staff: String, payment_method: String) {
    let template = include_str!("../../assets/mockups/sales_form.html").to_string();
    let template = template.replace("__CUSTOMER_NAME", &customer.name);
    let template = template.replace("__CUSTOMER_CONTACT", &customer.contact);
    let template = template.replace("__CUSTOMER_ADDRESS", &customer.address);
    let template = template.replace("__CUSTOMER_ID", &customer.id_num);
    let template = template.replace("__DEVICE_NAME", &device.name);
    let template = template.replace("__DEVICE_COLOR", &device.color);
    let template = template.replace("__DEVICE_LOCKED", &device.locked);
    let template = template.replace("__DEVICE_IMEI", &device.imei);
    let template = template.replace("__DEVICE_PRICE", &device.price);
    let template = template.replace("__PAYMENT_METHOD", &payment_method);
    let template = template.replace("__DATE", &date);
    let template = template.replace("__STAFF", &staff);

    // start a new thread and create the html_file in env::temp_dir()
    // then call open_html_file() to open the file in the default browser
    // if failes, print the error

    std::thread::spawn(move || {
        // create the file using std::fs::write, catch any errors
        let file_path = std::env::temp_dir().join("sales_form.html");
        match std::fs::write(&file_path, template) {
            Ok(_) => {
                // open the file in the default browser
                if let Err(e) = open_html_file(file_path) {
                    eprintln!("Failed to open the file: {}", e);
                    let _ = Notification::new()
                                .summary("Failed to load temporary form file")
                                .appname("Casdesk")
                                .show();
                }
            }
            Err(e) => {
                eprintln!("Failed to write the file: {}", e);
                let _ = Notification::new()
                                .summary("Failed to create temporary form file")
                                .appname("Casdesk")
                                .show();
            }
        }
    }); 
}
