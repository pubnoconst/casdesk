use dioxus::prelude::*;
const CSS :&str  = include_str!("home.css");


#[component]
pub fn Home() -> Element {
    let nv = navigator();
    rsx! {
        style  { "{CSS}" }
        div {
            class: "frame",
            div {
                id: "home-body",
                div { 
                    id: "title",
                    "Casdesk" 
                }
                div {
                    id: "button-row",
                    button { 
                        onclick: move |_| {
                            nv.push("/forms");
                        },
                        "Forms"
                    }
                    button {
                        onclick: move |_| {
                            nv.push("/quote");
                        }, 
                        "Quote"
                    }
                    button { 
                        onclick: move |_| {
                            nv.push("/adjust");
                        },
                        "Adjust POS"
                    }
                }
            }
        }
    }
}