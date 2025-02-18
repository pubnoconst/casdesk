#![windows_subsystem = "windows"]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::*;
use dioxus_free_icons::Icon;
mod formsview;
use formsview::*;
mod quoteview;
use quoteview::*;
mod adjustview;
use adjustview::*;
mod util;

// static CSS: Asset = asset!("/assets/main.css", CssAssetOptions::new().with_minify(true));
const CSS: &str = include_str!("../assets/main.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/forms")]
    Forms {},
    #[route("/quote")]
    Quote {},
    #[route("/adjust")]
    Adjust {}
}

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(dioxus::desktop::WindowBuilder::new().with_title("Casdesk 1.1.1")),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // document::Stylesheet{href: CSS}
        Router::<Route> {},
        style { "{CSS}" }
    }
}

#[component]
fn Home() -> Element {
    let nav = navigator();
    rsx! {
        body {
            div {
                id: "home-body",
                div {
                    id: "title",
                    h1 {
                        class: "title-text",
                        "Casdesk"
                    }
                }
                div {
                    id: "home-view-buttons",
                    button {
                        onclick: move |_| { nav.push(Route::Forms {}); },
                        Icon {
                            icon: FiPenTool
                        },
                        "Forms"
                    }
                    button {
                        onclick: move |_| { nav.push(Route::Quote {}); },
                        Icon {
                            icon: FiList
                        },
                        "Quote"
                    }
                    button {  
                        onclick: move |_| { nav.push(Route::Adjust { }); },
                        Icon {
                            icon: FiSliders
                        },
                        "Adjust POS"
                    }
                }
            }
        }
    }
}
