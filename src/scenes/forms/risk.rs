use std::{collections::HashMap, env, fs, rc::Rc};
use dioxus::prelude::*;
use super::io::open;

#[derive(PartialEq, Clone, Copy)]
pub enum RiskFormType {
    FragileScreen,
    BackGlass,
}

#[derive(PartialEq, Clone, Props)]
pub struct RiskFormProps {
    #[props(into)]
    kind: RiskFormType,
}

struct RiskFormArgs {
    customer_name: Rc<str>,
    device_model: Rc<str>,
}

impl RiskFormArgs {
    fn parse(data: HashMap<String, FormValue>) -> Option<Self> {
        Some(Self {
            customer_name: data.get("customer_name")?.first()?.as_str().into(),
            device_model: data.get("device_model")?.first()?.as_str().into(),
        })
    }

    fn print(&self, risk_form_type: RiskFormType) -> Result<(), Box<dyn std::error::Error>> {
        // Read the HTML template at compile time
        let template = match risk_form_type {
            RiskFormType::BackGlass => include_str!("back_glass_form.html"),
            RiskFormType::FragileScreen => include_str!("fragile_screen_form.html")
        };

        // Replace placeholders
        let output_html = template
            .replace("%LOGO_BANNER%", super::LOGO)
            .replace("%DEVICE_MODEL%", &self.device_model)
            .replace("%CUSTOMER_NAME%", &self.customer_name)
            .replace("%DATE%", &super::io::date::today().expect(""));

        // Create a temp directory path
        let mut temp_dir = env::temp_dir();
        temp_dir.push("risk_acknowledgement.html");

        // Write to the file
        fs::write(&temp_dir, output_html)?;

        // Open the file with the default system browser
        open(&temp_dir)?;

        Ok(())
    }
}

#[component]
pub fn RiskForm(props: RiskFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |e| {
                match RiskFormArgs::parse(e.data().values()) {
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
                input { r#type: "text", placeholder: "Customer Name", name: "customer_name" }
            }
            div {
                class: "form-row",
                label { "Device:" }
                input { r#type: "text", placeholder: "Google Pixle 9 Pro", name: "device_model" }
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