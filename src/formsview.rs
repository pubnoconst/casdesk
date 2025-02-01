use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiChevronLeft;
use dioxus_free_icons::Icon;
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