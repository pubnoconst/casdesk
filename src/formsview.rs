use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Forms() -> Element {
    let nav = navigator();
    rsx! {
        div {
            id: "forms-body",
            div {
                id: "forms-navigator",
                button {
                    id: "forms-back-button",
                    onclick: move |_| {nav.push(Route::Home {}); },
                    "Back"
                }
                div {
                    h2 {
                        id: "forms-h2",
                        "Forms"
                    }
                }
            }
            div {
                div {
                    id: "forms-buttons",
                    button {
                        "Sale of a device"
                    }
                    button {
                        "Purchase of a device"
                    }
                    button {
                        "Lease a device"
                    }
                }
            }
        }
    }
}