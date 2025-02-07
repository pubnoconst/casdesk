use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiChevronLeft;
use dioxus_free_icons::Icon;
use crate::util;
use crate::Route;
use crate::util::*;

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
            onsubmit: move |e| {
                e.prevent_default();
                let data: HashMap<String, FormValue> = e.data().values();

                fn extract_data(data: &std::collections::HashMap<String, FormValue>) -> Option<(Customer, SellableDevice, String, String)> {
                    let customer_name = data.get("customer_name")?.get(0)?;
                    let device_model = data.get("device_model")?.get(0)?;
                    let device_color = data.get("device_color")?.get(0)?;
                    let device_imei = data.get("device_imei")?.get(0)?;
                    let device_provider = data.get("device_provider")?.get(0)?;
                    let device_price = data.get("device_price")?.get(0)?;
                    let payment_method = data.get("payment_method")?.get(0)?;
                    let customers_contact_number = data.get("customers_contact_number")?.get(0)?;
                    let customer_addr = data.get("customer_addr")?.get(0)?;
                    let customer_id = data.get("customer_id")?.get(0)?;

                    let stuff_name = data.get("stuff_name")?.get(0)?.to_owned();
                    let date_of_sale = data.get("date_of_sale")?.get(0)?.to_owned();
                
                    let customer = Customer::new(customer_name, customers_contact_number, customer_addr, customer_id);
                    let device = SellableDevice::new(device_model, device_color, device_provider, device_imei, device_price, payment_method);
                
                    Some((customer, device, stuff_name, date_of_sale))
                }           
                if let Some((customer, device, stuff_name, date_of_sale)) = extract_data(&data) {
                    println!("Customer: {:?}", customer);
                    println!("Device: {:?}", device);
                    println!("Staff: {}", stuff_name);
                    println!("Date: {}", date_of_sale);
                } else {
                    eprintln!("Error: Could not extract data from the sales form");
                }

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
                label { "Date:" }
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
            }
            div {
                class: "form-submit-button-container",
                button { 
                    class: "encouraged-button",
                    class: "form-submit-button",
                    r#type: "submit", 
                    "Confirm" 
                }
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
                label { "Date:" }
                input { r#type: "text", name: "date_of_sale", placeholder: "MM/DD/YY" }
            }
            div {
                class: "form-submit-button-container",
                button { 
                    class: "encouraged-button",
                    class: "form-submit-button",
                    r#type: "submit", 
                    "Confirm" 
                }
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
            onsubmit: move |_| {
                
            },
            // div {
            //     h4 {
            //         "Device information"
            //     }
            // }
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
                label { "IMEI/Serial number:" }
                input { r#type: "text", name: "device_imei", placeholder: "*#06*#" }
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
            // div {
            //     h4 {
            //         "Borrower's information"
            //     }
            // }
            div {
                class: "form-row",
                label { "Borrower's Name:" }
                input { r#type: "text", name: "borrower_name", placeholder: "Enter borrower name" }
            }
            div {
                class: "form-row",
                label { "Borrower's contact number:" }
                input { r#type: "text", name: "borrowers_contact_number", placeholder: "Enter phone number" }
            }
            div {
                class: "form-row",
                label { "Borrower's address:" }
                input { r#type: "text", name: "seller_addr", placeholder: "Enter seller's address" }
            }
            div {
                class: "form-row",
                label { "Borrower's ID number:" }
                input { r#type: "text", name: "seller_id", placeholder: "Enter customer ID" }
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
                    class: "form-submit-button",
                    r#type: "submit", 
                    "Confirm" 
                }
            }
        }
    }
}