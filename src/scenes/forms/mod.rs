use dioxus::prelude::*;

#[component]
pub fn Forms() -> Element {
    rsx! {
        div {
            class: "frame",
            super::NavBar { page_title: "Forms" }
            div {
                "placeholder"
            }
        }
    }
}