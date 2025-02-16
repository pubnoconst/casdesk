use notify_rust::Notification;
use std::path::PathBuf;
use std::process::Command;

use super::*;

const LOGO_BANNER: &[u8] = include_bytes!("../../assets/logobanner.png");

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

fn generate_and_open_form(template: String, file_name: String) {
    std::thread::spawn(move || {
        let file_path = std::env::temp_dir().join(file_name);
        match std::fs::write(&file_path, template) {
            Ok(_) => {
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

fn replace_placeholders<S: AsRef<str>>(template: &str, replacements: &[(&str, S)]) -> String {
    let template = template.to_string();
    let img = &rbase64::encode(LOGO_BANNER);
    let mut template = template.replace("__LOGO_BANNER", img);
    for (placeholder, value) in replacements {
        template = template.replace(placeholder, value.as_ref());
    }
    template
}

pub fn refurbished_device_sale_form(
    customer: Customer,
    device: SellableDevice,
    date: String,
    staff: String,
    payment_method: String,
) {
    let template =
        include_str!("../../assets/mockups/refurbished_device_sale_form.html").to_string();
    let replacements = vec![
        ("__CUSTOMER_NAME", &customer.name as &str),
        ("__CUSTOMER_CONTACT", &customer.contact as &str),
        ("__CUSTOMER_ADDRESS", &customer.address as &str),
        ("__CUSTOMER_ID", &customer.id_num as &str),
        ("__DEVICE_NAME", &device.name as &str),
        ("__DEVICE_COLOR", &device.color as &str),
        ("__DEVICE_LOCKED", &device.locked as &str),
        ("__DEVICE_IMEI", &device.imei as &str),
        ("__DEVICE_PRICE", &device.price as &str),
        ("__PAYMENT_METHOD", &payment_method as &str),
        ("__DATE", &date as &str),
        ("__STAFF", &staff as &str),
    ];

    let filled_template = replace_placeholders(&template, &replacements);
    generate_and_open_form(filled_template, "sales_form.html".to_string());
}

pub fn new_device_sale_form(
    customer: Customer,
    device: SellableDevice,
    date: String,
    staff: String,
    payment_method: String,
) {
    let template = include_str!("../../assets/mockups/new_device_sale_form.html").to_string();
    let replacements = vec![
        ("__CUSTOMER_NAME", &customer.name as &str),
        ("__CUSTOMER_CONTACT", &customer.contact as &str),
        ("__CUSTOMER_ADDRESS", &customer.address as &str),
        ("__CUSTOMER_ID", &customer.id_num as &str),
        ("__DEVICE_NAME", &device.name as &str),
        ("__DEVICE_COLOR", &device.color as &str),
        ("__DEVICE_LOCKED", &device.locked as &str),
        ("__DEVICE_IMEI", &device.imei as &str),
        ("__DEVICE_PRICE", &device.price as &str),
        ("__PAYMENT_METHOD", &payment_method),
        ("__DATE", &date),
        ("__STAFF", &staff),
    ];

    let filled_template = replace_placeholders(&template, &replacements);
    generate_and_open_form(filled_template, "sales_form.html".to_string());
}

pub fn purchase_form(
    customer: Customer,
    device: PurchasedDevice,
    price: String,
    staff: String,
    date: String,
    notes: String,
) {
    let template = include_str!("../../assets/mockups/purchase_form.html").to_string();
    let replacements = vec![
        ("__SELLER_NAME", &customer.name as &str),
        ("__PRICE", &price as &str),
        ("__SELLER_CONTACT", &customer.contact as &str),
        ("__SELLER_ADDRESS", &customer.address as &str),
        ("__SELLER_ID", &customer.id_num as &str),
        ("__DEVICE_NAME", &device.name as &str),
        ("__DEVICE_MEMORY", &device.memory as &str),
        ("__DEVICE_COLOR", &device.color as &str),
        ("__DEVICE_LOCKED", &device.locked as &str),
        ("__DEVICE_IMEI", &device.imei as &str),
        ("__DEVICE_PRICE", &price as &str),
        ("__NOTES", &notes),
        ("__DATE", &date),
        ("__STAFF", &staff),
    ];

    let filled_template = replace_placeholders(&template, &replacements);
    generate_and_open_form(filled_template, "purchase_form.html".to_string());
}

pub fn lease_form(
    borrower: &Customer,
    device: &LeasedDevice,
    accessories: String,
    staff: String,
    date: String,
) {
    let template = include_str!("../../assets/mockups/lease_device_form.html");
    let replacements = vec![
        ("__BORROWER_NAME", &borrower.name as &str),
        ("__DEVICE_STORAGE", &device.storage as &str),
        ("__DEVICE_COLOR", &device.color as &str),
        ("__DEVICE_IMEI", &device.imei as &str),
        ("__DEVICE_CONDITION", &device.condition as &str),
        ("__ACCESSORIES", &accessories),
        ("__BORROWER_ADDRESS", &borrower.address),
        ("__BORROWER_CONTACT", &borrower.contact),
        ("__BORROWER_ID", &borrower.id_num),
        ("__DATE", &date),
        ("__STAFF", &staff),
    ];
    let filled_template = replace_placeholders(template, &replacements);
    generate_and_open_form(filled_template, "leaseform.html".to_string());
}
