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
                .with_window(dioxus::desktop::WindowBuilder::new().with_title("Casdesk 1.1.2")),
        )
        .launch(App);
}

// debug build only
#[cfg(debug_assertions)]
static CSS: Asset = asset!("/assets/main.css", CssAssetOptions::new().with_minify(true));

// release build only
#[cfg(not(debug_assertions))]
const CSS: &str = include_str!("../assets/main.css");

#[component]
fn App() -> Element {
    let stylesheet = if cfg!(debug_assertions) {
        rsx! { document::Stylesheet { href: CSS } }
    } else {
        rsx! { style { "{CSS}" } }
    };

    rsx! {
        {stylesheet}
        Router::<Route> {}
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
