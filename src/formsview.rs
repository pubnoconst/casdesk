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
                label { "Device Model:" }
                input { r#type: "text", name: "device_model", placeholder: "Enter device model" }
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
                label { "Customer Name:" }
                input { r#type: "text", name: "customer_name", placeholder: "Enter customer name" }
            }
            div {
                class: "form-row",
                label { "Device Model:" }
                input { r#type: "text", name: "device_model", placeholder: "Enter device model" }
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