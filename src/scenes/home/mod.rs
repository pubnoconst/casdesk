use dioxus::prelude::*;

const CSS :&str  = include_str!("home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        style  { "{CSS}" }
        div {
            id: "home-body",
            p { "Casdesk" }
            div {
                id: "button-row",
                button { 
                    "Forms"
                }
                button { 
                    "Quote"
                }
                button { 
                    "Adjust POS"
                }
            }
        }
    }
}