use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum RiskFormType {
    FragileScreen,
    BackGlass,
}

#[derive(PartialEq, Clone, Props)]
pub struct RiskFormProps {
    #[props(into)]
    pub kind: RiskFormType,
}

pub struct RiskFormArgs {
    customer_name: Arc<str>,
    device_model: Arc<str>,
}

impl RiskFormArgs {
    pub fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        Some(Self {
            customer_name: data.get("customer_name")?.first()?.as_str().into(),
            device_model: data.get("device_model")?.first()?.as_str().into(),
        })
    }
}

#[component]
pub fn RiskForm(props: RiskFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |e| {
                match RiskFormArgs::parse(e.data().values()) {
                    Some(args) => {}
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
                input { r#type: "text", placeholder: "Customer Name", name: "customer_name" }
            }
            div {
                class: "form-row",
                label { "Device:" }
                input { r#type: "text", placeholder: "Google Pixle 9 Pro", name: "device_model" }
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