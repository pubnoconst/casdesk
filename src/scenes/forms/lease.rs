use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;

pub struct LeaseFormArgs {
    device_model: Arc<str>,
    device_color: Arc<str>,
    device_storage: Arc<str>,
    device_imei: Arc<str>,
    device_condition: Arc<str>,
    accessories: Arc<str>,
    borrower_name: Arc<str>,
    borrower_contact_number: Arc<str>,
    borrower_addr: Arc<str>,
    borrower_id: Arc<str>,
    staff_name: Arc<str>,
    date: Arc<str>,
}

impl LeaseFormArgs {
    pub fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        Some(Self {
            device_model: data.get("device_model")?.first()?.as_str().into(),
            device_color: data.get("device_color")?.first()?.as_str().into(),
            device_storage: data.get("device_storage")?.first()?.as_str().into(),
            device_imei: data.get("device_imei")?.first()?.as_str().into(),
            device_condition: data.get("device_condition")?.first()?.as_str().into(),
            accessories: data.get("accessories")?.first()?.as_str().into(),
            borrower_name: data.get("borrower_name")?.first()?.as_str().into(),
            borrower_contact_number: data.get("borrower_contact_number")?.first()?.as_str().into(),
            borrower_addr: data.get("borrower_addr")?.first()?.as_str().into(),
            borrower_id: data.get("borrower_id")?.first()?.as_str().into(),
            staff_name: data.get("staff_name")?.first()?.as_str().into(),
            date: data.get("date")?.first()?.as_str().into(),
        })
    }
}

#[component]
pub fn Lease() -> Element {
    rsx! {
       form {
            class: "form",
            onsubmit: move |_| {
                
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
                label { "Device Storage (GB):" }
                input { r#type: "number", name: "device_storage", placeholder: "64" }
            }
            div {
                class: "form-row",
                label { "IMEI/Serial number:" }
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
                label { "Borrower's ID number:" }
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
                input { r#type: "text", name: "date", placeholder: "MM/DD/YY" }
            } 
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