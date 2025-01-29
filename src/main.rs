use dioxus::prelude::*;
mod formsview;
use formsview::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/forms")]
    Forms {}
}

fn main() {
    LaunchBuilder::new().with_cfg(
        dioxus::desktop::Config::default()
        .with_menu(None)
        .with_window(dioxus::desktop::WindowBuilder::new().with_title("Casdesk"))
    ).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: CSS}
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let nav = navigator();
    rsx!{
        body {
            div {
                id: "home-body",
                div { 
                    id: "title",
                    h1 { "Casdesk" }
                }
                div { 
                    id: "home-view-buttons",
                    button {
                        onclick: move |_| { nav.push(Route::Forms {}); },
                        "Forms"
                    }         
                    button {  
                        "Quote"
                    }       
                    button {
                        "Calculator"
                    }
                }
            }
        }
    }
}

