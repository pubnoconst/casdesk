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

#[component]
pub fn RiskForm(props: RiskFormProps) -> Element {
    rsx! {
        form {
            class: "form",
            onsubmit: move |_| {

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