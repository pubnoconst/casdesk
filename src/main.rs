use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: CSS}
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

