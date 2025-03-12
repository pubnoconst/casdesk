use dioxus::prelude::*;
use std::{collections::HashMap, env, fs, rc::Rc};
use super::io::open;

#[derive(PartialEq, Clone, Copy)]
pub enum SaleType {
    Refurbished,
    New,
}

#[derive(PartialEq, Clone, Props)]
pub struct SaleFormProps {
    #[props(into)]
    kind: SaleType,
}

struct SaleFormArgs {
    customer_name: Rc<str>,
    device_model: Rc<str>,
    device_color: Rc<str>,
    device_imei: Rc<str>,
    device_provider: Rc<str>,
    device_price: Rc<str>,
    payment_method: Rc<str>,
    customers_contact_number: Rc<str>,
    customer_addr: Rc<str>,
    customer_id: Rc<str>,
    staff_name: Rc<str>,
    date_of_sale: Rc<str>,
}

impl SaleFormArgs {
    fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
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

    fn print(&self, sale_type: SaleType) -> Result<(), Box<dyn std::error::Error>> {
        // Read the HTML template at compile time
        let template = match sale_type {
            SaleType::Refurbished => include_str!("refurbished_device_sale_form.html"),
            SaleType::New => include_str!("new_device_sale_form.html")
        } ;

        // Replace placeholders
        let output_html = template
            .replace("%LOGO_BANNER%", super::LOGO)
            .replace("%CUSTOMER_NAME%", &self.customer_name)
            .replace("%DEVICE_NAME%", &self.device_model)
            .replace("%DEVICE_COLOR%", &self.device_color)
            .replace("%DEVICE_LOCKED%", &self.device_provider)
            .replace("%DEVICE_IMEI%", &self.device_imei)
            .replace("%DEVICE_PRICE%", &self.device_price)
            .replace("%PAYMENT_METHOD%", &self.payment_method)
            .replace("%CUSTOMER_CONTACT%", &self.customers_contact_number)
            .replace("%CUSTOMER_ADDRESS%", &self.customer_addr)
            .replace("%CUSTOMER_ID%", &self.customer_id)
            .replace("%DATE%", &self.date_of_sale)
            .replace("%STAFF%", &self.staff_name);

        // Create a temp directory path
        let mut temp_dir = env::temp_dir();
        temp_dir.push("sale_contract.html");

        // Write to the file
        fs::write(&temp_dir, output_html)?;

        // Open the file with the default system browser
        open(&temp_dir)?;

        Ok(())
    }
}

#[component]
pub fn Sale(props: SaleFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            autocomplete: "off",
            onsubmit: move |e| {
                match SaleFormArgs::parse(e.data().values()) {
                    Some(args) => {
                        if args.print(props.kind).is_err() {
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
                label { "Price (AUD):" }
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
                label { "Customer address:" }
                input { r#type: "text", name: "customer_addr", placeholder: "Enter customer's address" }
            }
            div {
                class: "form-row",
                label { "Customer ID:" }
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
                input { r#type: "text", name: "date_of_sale", placeholder: super::io::date::today().expect("").as_str() }
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
