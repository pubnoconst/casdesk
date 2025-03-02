use std::rc::Rc;
use dioxus::prelude::*;

pub mod home;
pub mod forms;
pub mod quote;
pub mod adjust;

pub const GLOBAL_CSS :&str  = include_str!("global.css");

#[derive(PartialEq, Clone, Props)]
pub struct NavBarProps {
    #[props(into)]
    page_title: Rc<str>,
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    let nv = navigator();
    rsx! {
        div {
            class: "navbar",
            div {
                class: "navbar-left",
                button { 
                    onclick: move |_| {
                        nv.push("/");
                    },
                    "Home"
                }
            }
            div {
                class: "navbar-right",
               "{props.page_title}" 
            }
        }
    }
}