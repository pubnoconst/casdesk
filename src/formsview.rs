use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Forms() -> Element {
    let nav = navigator();
    rsx! {
        div {
            button {
                onclick: move |_| {nav.push(Route::Home {}); },
                "Back"
            }
        }
        div {
            id: "forms-body",
            div {
                h1 {
                    "Forms"
                }
            }
            div {
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