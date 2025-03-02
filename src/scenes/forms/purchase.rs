use dioxus::prelude::*;

#[component]
pub fn Purchase() -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |_| {

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
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
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