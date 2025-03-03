use std::{collections::HashMap, env, fs, sync::Arc};
use dioxus::prelude::*;
use super::io::open;

struct PurchaseFormArgs {
    seller_name: Arc<str>,
    device_model: Arc<str>,
    device_color: Arc<str>,
    device_memory: Arc<str>,
    device_imei: Arc<str>,
    device_provider: Arc<str>,
    purchase_price: Arc<str>,
    sellers_contact_number: Arc<str>,
    seller_addr: Arc<str>,
    seller_id: Arc<str>,
    staff_name: Arc<str>,
    date_of_sale: Arc<str>,
    notes: Arc<str>,
}

impl PurchaseFormArgs {
    fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        Some(Self {
            seller_name: data.get("seller_name")?.first()?.as_str().into(),
            device_model: data.get("device_model")?.first()?.as_str().into(),
            device_color: data.get("device_color")?.first()?.as_str().into(),
            device_memory: data.get("device_memory")?.first()?.as_str().into(),
            device_imei: data.get("device_imei")?.first()?.as_str().into(),
            device_provider: data.get("device_provider")?.first()?.as_str().into(),
            purchase_price: data.get("purchase_price")?.first()?.as_str().into(),
            sellers_contact_number: data.get("sellers_contact_number")?.first()?.as_str().into(),
            seller_addr: data.get("seller_addr")?.first()?.as_str().into(),
            seller_id: data.get("seller_id")?.first()?.as_str().into(),
            staff_name: data.get("staff_name")?.first()?.as_str().into(),
            date_of_sale: data.get("date_of_sale")?.first()?.as_str().into(),
            notes: data.get("notes")?.first()?.as_str().into(),
        })
    }

    fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Read the HTML template at compile time
        let template = include_str!("purchase_form.html");

        // Replace placeholders
        let output_html = template
            .replace("%LOGO_BANNER%", &super::io::logo_bytes())
            .replace("%SELLER_NAME%", &self.seller_name)
            .replace("%DEVICE_NAME%", &self.device_model)
            .replace("%DEVICE_COLOR%", &self.device_color)
            .replace("%DEVICE_MEMORY%", &self.device_memory)
            .replace("%DEVICE_LOCKED%", &self.device_provider)
            .replace("%DEVICE_IMEI%", &self.device_imei)
            .replace("%PRICE%", &self.purchase_price)
            .replace("%SELLER_CONTACT%", &self.sellers_contact_number)
            .replace("%SELLER_ADDRESS%", &self.seller_addr)
            .replace("%CUSTOMER_ID%", &self.seller_id)
            .replace("%DATE%", &self.date_of_sale)
            .replace("%STAFF%", &self.staff_name)
            .replace("%NOTES%", &self.notes);

        // Create a temp directory path
        let mut temp_dir = env::temp_dir();
        temp_dir.push("purchase_contract.html");

        // Write to the file
        fs::write(&temp_dir, output_html)?;

        // Open the file with the default system browser
        open(&temp_dir)?;

        Ok(())
    }
}

#[component]
pub fn Purchase() -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |e| {
                match PurchaseFormArgs::parse(e.data().values()) {
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
                label { "Seller's Name:" }
                input { r#type: "text", name: "seller_name", placeholder: "Enter seller's name" }
            }
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
                label { "Memory (in GB):" }
                input { r#type: "number", name: "device_memory", placeholder: "256" }
            }
            div {
                class: "form-row",
                label { "IMEI (or the like):" }
                input { r#type: "text", name: "device_imei", placeholder: "*#06#" }
            }
            div {
                class: "form-row",
                label { "Locked to provider:" }
                input { r#type: "text", name: "device_provider", placeholder: "Unlocked/Optus..." }
            }
            div {
                class: "form-row",
                label { "Purchase price AUD $:" }
                input { r#type: "number", name: "purchase_price", placeholder: "Enter purchase price" }
            }
            div {
                class: "form-row",
                label { "Contact:" }
                input { r#type: "text", name: "sellers_contact_number", placeholder: "Seller's phone number" }
            }
            div {
                class: "form-row",
                label { "Seller's address:" }
                input { r#type: "text", name: "seller_addr", placeholder: "Enter seller's address" }
            }
            div {
                class: "form-row",
                label { "Seller's ID number:" }
                input { r#type: "text", name: "seller_id", placeholder: "Enter customer ID" }
            }
            div {
                class: "form-row",
                label { "Staff name:" }
                input { r#type: "text", name: "staff_name", placeholder: "Enter staff name" }
            }
            div {
                class: "form-row",
                label { "Date:" }
                input { r#type: "text", name: "date_of_sale", placeholder: super::io::date::today().expect("").as_str() }
            }
            div {
                class: "form-row",
                label { "Notes for Office:" }
                input { r#type: "text", name: "notes", placeholder: "Is any repairs needed?" }
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