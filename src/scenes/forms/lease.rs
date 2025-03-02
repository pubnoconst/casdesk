use dioxus::prelude::*;

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