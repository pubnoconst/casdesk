use std::{collections::HashMap, env, fs, rc::Rc};
use dioxus::prelude::*;
use super::io::open;

struct LeaseFormArgs {
    device_model: Rc<str>,
    device_color: Rc<str>,
    device_storage: Rc<str>,
    device_imei: Rc<str>,
    device_condition: Rc<str>,
    accessories: Rc<str>,
    borrower_name: Rc<str>,
    borrower_contact_number: Rc<str>,
    borrower_addr: Rc<str>, 
    borrower_id: Rc<str>,
    staff_name: Rc<str>,
    date: Rc<str>,
}

impl LeaseFormArgs {
    fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        Some(Self {
            device_model: data.get("device_model")?.first()?.as_str().into(),
            device_color: data.get("device_color")?.first()?.as_str().into(),
            device_storage: data.get("device_storage")?.first()?.as_str().into(),
            device_imei: data.get("device_imei")?.first()?.as_str().into(),
            device_condition: data.get("device_condition")?.first()?.as_str().into(),
            accessories: data.get("accessories")?.first()?.as_str().into(),
            borrower_name: data.get("borrower_name")?.first()?.as_str().into(),
            borrower_contact_number: data
                .get("borrower_contact_number")?
                .first()?
                .as_str()
                .into(),
            borrower_addr: data.get("borrower_addr")?.first()?.as_str().into(),
            borrower_id: data.get("borrower_id")?.first()?.as_str().into(),
            staff_name: data.get("staff_name")?.first()?.as_str().into(),
            date: data.get("date")?.first()?.as_str().into(),
        })
    }

    fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Read the HTML template at compile time
        let template = include_str!("lease_device_form.html");

        // Replace placeholders
        let output_html = template
            .replace("%LOGO_BANNER%", super::LOGO)
            .replace("%DEVICE_NAME%", &self.device_model)
            .replace("%DEVICE_COLOR%", &self.device_color)
            .replace("%DEVICE_STORAGE%", &self.device_storage)
            .replace("%DEVICE_IMEI%", &self.device_imei)
            .replace("%DEVICE_CONDITION%", &self.device_condition)
            .replace("%ACCESSORIES%", &self.accessories)
            .replace("%BORROWER_NAME%", &self.borrower_name)
            .replace("%BORROWER_CONTACT%", &self.borrower_contact_number)
            .replace("%BORROWER_ADDRESS%", &self.borrower_addr)
            .replace("%BORROWER_ID%", &self.borrower_id)
            .replace("%DATE%", &self.date)
            .replace("%STAFF%", &self.staff_name);

        // Create a temp directory path
        let mut temp_dir = env::temp_dir();
        temp_dir.push("lease_agreement.html");

        // Write to the file
        fs::write(&temp_dir, output_html)?;

        // Open the file with the default system browser
        open(&temp_dir)?;

        Ok(())
    }
}

#[component]
pub fn Lease() -> Element {
    rsx! {
       form {
            autocomplete: "off",
            class: "form",
            onsubmit: move |e| {
                match LeaseFormArgs::parse(e.data().values()) {
                    Some(args) => {
                        if args.print().is_err() {
                            let _ = notify_rust::Notification::new()
                                        .appname("Casdesk")
                                        .body("Error creating contract form")
                                        .show();
                        }
                    }
                    _ => {
                        let _ = notify_rust::Notification::new()
                            .appname("Casdesk")
                            .body("Error reading form values")
                            .show();
                    }
                }
            },
            div {
                class: "form-row",
                label { "Device:" }
                input { r#type: "text", name: "device_model", placeholder: "Name, model, model number" }
            }
            div {
                class: "form-row",
                label { "Device Color:" }
                input { r#type: "text", name: "device_color", placeholder: "Enter device color" }
            }
            div {
                class: "form-row",
                label { "Storage (GB):" }
                input { r#type: "number", name: "device_storage", placeholder: "64" }
            }
            div {
                class: "form-row",
                label { "IMEI/Serial:" }
                input { r#type: "text", name: "device_imei", placeholder: "*#06#" }
            }
            div {
                class: "form-row",
                label { "Device condition:" }
                input { r#type: "text", name: "device_condition", placeholder: "Describe the condition of the device" }
            }
            div {
                class: "form-row",
                label { "Accessories:" }
                input { r#type: "text", name: "accessories", placeholder: "None/Case/Screen Protector" }
            }
            div {
                class: "form-row",
                label { "Borrower's name:" }
                input { r#type: "text", name: "borrower_name", placeholder: "Enter borrower name" }
            }
            div {
                class: "form-row",
                label { "Contact:" }
                input { r#type: "text", name: "borrower_contact_number", placeholder: "Enter borrower's phone number" }
            }
            div {
                class: "form-row",
                label { "Borrower's address:" }
                input { r#type: "text", name: "borrower_addr", placeholder: "Enter borrower's address" }
            }
            div {
                class: "form-row",
                label { "Borrower's ID:" }
                input { r#type: "text", name: "borrower_id", placeholder: "Enter borrower's ID" }
            }
            div {
                class: "form-row",
                label { "Staff name:" }
                input { r#type: "text", name: "staff_name", placeholder: "Enter staff name" }
            }
            div {
                class: "form-row",
                label { "Date:" }
                input { r#type: "text", name: "date", placeholder: super::io::date::today().expect("").as_str() }
            }
            div {
                class: "form-submit-button-container",
                button {
                    class: "encouraged-button",
                    r#type: "submit",
                    "Confirm"
                }
            }
       }
    }
}
