use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiChevronLeft;
use dioxus_free_icons::Icon;
use crate::Route;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab {
    Sale,
    Purchase,
    Lease,
}

#[component]
pub fn Forms() -> Element {
    let nav = navigator();
    rsx! {
        div {
            class: "primary-body",
            div {
                class: "primary-navigator",
                button {
                    class: "back-button",
                    onclick: move |_| {
                        nav.push(Route::Home {}); 
                    },
                    Icon {
                        icon: FiChevronLeft
                    }
                    "Back"
                }
                div {
                    h2 {
                        class: "title-text",
                        "Forms"
                    }
                }
            }
            FormBody {}
        }
    }
}

#[component]
fn FormBody() -> Element {
    let mut current_tab = use_signal(|| Tab::Sale);
    rsx! {
        div {
            id: "forms-body",
            div {
                id: "forms-buttons",
                button {
                    onclick: move |_| current_tab.set(Tab::Sale),
                    "Sale of a device"
                }
                button {
                    onclick: move |_| current_tab.set(Tab::Purchase),
                    "Purchase of a device"
                }
                button {
                    onclick: move |_| current_tab.set(Tab::Lease),
                    "Lease a device"
                }
            }
            div {
                class: "form-container",
                match *(current_tab.read()) {
                    Tab::Sale => rsx! { SaleForm {} },
                    Tab::Purchase => rsx! { PurchaseForm {} },
                    Tab::Lease => rsx! { LeaseForm {} },
                }
            }
        }
    }
}

#[component]
fn SaleForm() -> Element {
    rsx! {
        h2 { "Sale Form" }
        form {
            class: "form-div",
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
                input { r#type: "text", name: "device_imei", placeholder: "*#06*#" }
            }
            div {
                class: "form-row",
                label { "Locked to provider:" }
                input { r#type: "text", name: "device_provider", placeholder: "Unlocked/Optus..." }
            }
            div {
                class: "form-row",
                label { "Device Price AUD $:" }
                input { r#type: "text", name: "device_price", placeholder: "Enter device price" }
            }
            div {
                class: "form-row",
                label { "Payment Method:" }
                input { r#type: "text", name: "payment_method", placeholder: "EFTPOS/Cash" }
            }
            div {
                class: "form-row",
                label { "Customer's contact number:" }
                input { r#type: "text", name: "customers_contact_number", placeholder: "Enter phone number" }
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
                label { "date" }
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
            }
        }
    }
}

#[component]
fn PurchaseForm() -> Element {
    rsx! {
        h2 { "Purchase Form" }
        form {
            class: "form-div",
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
                label { "IMEI (or the like):" }
                input { r#type: "text", name: "device_imei", placeholder: "*#06*#" }
            }
            div {
                class: "form-row",
                label { "Locked to provider:" }
                input { r#type: "text", name: "device_provider", placeholder: "Unlocked/Optus..." }
            }
            div {
                class: "form-row",
                label { "Purchase price AUD $:" }
                input { r#type: "text", name: "purchase_price", placeholder: "Enter purchase price" }
            }
            div {
                class: "form-row",
                label { "Seller's contact number:" }
                input { r#type: "text", name: "sellers_contact_number", placeholder: "Enter phone number" }
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
                input { r#type: "text", name: "stuff_name", placeholder: "Enter staff name" }
            }
            div {
                class: "form-row",
                label { "date" }
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
            }
        }
    }
}

#[component]
fn LeaseForm() -> Element {
    rsx! {
        h2 { "Lease Form" }
        form {
            class: "form-div",
            div {
                class: "form-row",
                label { "Customer Name:" }
                input { r#type: "text", name: "customer_name", placeholder: "Enter customer name" }
            }
            div {
                class: "form-row",
                label { "Device Model:" }
                input { r#type: "text", name: "device_model", placeholder: "Enter device model" }
            }
            // button { r#type: "submit", "Submit" }
        }
    }
}