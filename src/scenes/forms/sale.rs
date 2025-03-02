use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum SaleType {
    Refurbished,
    New,
}

#[derive(PartialEq, Clone, Props)]
pub struct SaleFormProps {
    #[props(into)]
    kind: SaleType,
}

#[component]
pub fn Sale(props: SaleFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |_| {

            },
            div {
                class: "form-row",
                label { "Customer Name:" }
                input { r#type: "text", name: "customer_name", placeholder: "Enter customer name" }
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
                label { "Device Price AUD $:" }
                input { r#type: "number", name: "device_price", placeholder: "Enter device price" }
            }
            div {
                class: "form-row",
                label { "Payment Method:" }
                input { r#type: "text", name: "payment_method", placeholder: "EFTPOS/Cash" }
            }
            div {
                class: "form-row",
                label { "Contact:" }
                input { r#type: "text", name: "customers_contact_number", placeholder: "Customer's phone number" }
            }
            div {
                class: "form-row",
                label { "Customer's address:" }
                input { r#type: "text", name: "customer_addr", placeholder: "Enter customer's address" }
            }
            div {
                class: "form-row",
                label { "Customer's ID number:" }
                input { r#type: "text", name: "customer_id", placeholder: "Enter customer's ID" }
            }
            div {
                class: "form-row",
                label { "Staff name:" }
                input { r#type: "text", name: "stuff_name", placeholder: "Enter staff name" }
            }
            div {
                class: "form-row",
                label { "Date:" }
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
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
