use dioxus::prelude::*;

#[component]
pub fn Adjust() -> Element {
    rsx! {
        div {
            class: "frame",
            super::NavBar { page_title: "Adjust POS" }
            div {
                "placeholder"
            }
        }
    }
}