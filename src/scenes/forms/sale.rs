use dioxus::prelude::*;
use std::{collections::HashMap, sync::Arc};

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

pub struct SaleFormArgs {
    customer_name: Arc<str>,
    device_model: Arc<str>,
    device_color: Arc<str>,
    device_imei: Arc<str>,
    device_provider: Arc<str>,
    device_price: Arc<str>,
    payment_method: Arc<str>,
    customers_contact_number: Arc<str>,
    customer_addr: Arc<str>,
    customer_id: Arc<str>,
    staff_name: Arc<str>,
    date_of_sale: Arc<str>,
}

impl SaleFormArgs {
    pub fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        let customer_name = data.get("customer_name")?.first()?.as_str();
        let device_color = data.get("device_color")?.first()?.as_str();
        let device_model = data.get("device_model")?.first()?.as_str();
        let device_imei = data.get("device_imei")?.first()?.as_str();
        let device_provider = data.get("device_provider")?.first()?.as_str();
        let device_price = data.get("device_price")?.first()?.as_str();
        let payment_method = data.get("payment_method")?.first()?.as_str();
        let customers_contact_number = data.get("customers_contact_number")?.first()?.as_str();
        let customer_addr = data.get("customer_addr")?.first()?.as_str();
        let customer_id = data.get("customer_id")?.first()?.as_str();
        let staff_name = data.get("staff_name")?.first()?.as_str();
        let date_of_sale = data.get("date_of_sale")?.first()?.as_str();

        Some(Self {
            customer_name: customer_name.into(),
            device_model: device_model.into(),
            device_color: device_color.into(),
            device_imei: device_imei.into(),
            device_provider: device_provider.into(),
            device_price: device_price.into(),
            payment_method: payment_method.into(),
            customers_contact_number: customers_contact_number.into(),
            customer_addr: customer_addr.into(),
            customer_id: customer_id.into(),
            staff_name: staff_name.into(),
            date_of_sale: date_of_sale.into(),
        })
    }
}

#[component]
pub fn Sale(props: SaleFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |e| {
                let args = SaleFormArgs::parse(e.data().values());
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
                input { r#type: "text", name: "staff_name", placeholder: "Enter staff name" }
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
