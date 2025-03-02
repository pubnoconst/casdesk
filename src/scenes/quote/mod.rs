use dioxus::prelude::*;

#[component]
pub fn Quote() -> Element {
    rsx! {
        div {
            class: "frame",
            super::NavBar { page_title: "Quote" }
            div {
                "placeholder"
            }
        }
    }
}