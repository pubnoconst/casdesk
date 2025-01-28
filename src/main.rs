use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home {}
}

fn main() {
    dioxus::launch(App);
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
    rsx!{
        body {
            div {
                id: "home-body",
                div { 
                    id: "title",
                    h1 { "Casdesk." }
                }
                div { 
                    id: "home-view-buttons",
                    button {
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

