use std::rc::Rc;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdArrowLeft, Icon};

pub mod home;
pub mod forms;
pub mod quote;
pub mod adjust;

pub const GLOBAL_CSS :&str  = include_str!("global.css");
pub const FIGTREE_FONT_BYTES: &str = include_str!("../../assets/output/Figtree.ttf.bin"); // Your font file

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
                // class: "navbar-left",
                button { 
                    // class: "back-button",
                    onclick: move |_| {
                        nv.push("/");
                    },
                    Icon {
                        icon: LdArrowLeft,
                        height: 13,
                    }
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