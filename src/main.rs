use dioxus::{desktop::LogicalSize, prelude::*};
mod scenes;
use scenes::{home::*, forms::*, quote::*, adjust::*};

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

#[component]
fn App() -> Element {
     rsx! {
        style { "{scenes::GLOBAL_CSS}"  }
        Router::<Route> {}
     }
}

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("Casdesk 1.1.2")
                        .with_inner_size(LogicalSize::new(750.0, 750.0)),
                ),
        )
        .launch(App);
}
