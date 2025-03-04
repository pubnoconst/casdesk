#![windows_subsystem = "windows"]

use dioxus::{desktop::LogicalSize, prelude::*};
mod scenes;
use scenes::{adjust::*, forms::*, home::*, quote::*, GLOBAL_CSS};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/forms")]
    Forms {},
    #[route("/quote")]
    Quote {},
    #[route("/adjust")]
    Adjust {},
}

#[component]
fn App() -> Element {
    let font_base64 = rbase64::encode(scenes::FONT_BYTES);
    let font_face = format!(
        "@font-face {{ font-family: 'Jost'; src: url('data:font/ttf;base64,{}') format('truetype'); }}",
        font_base64
    );
    let global_css = format!("{}\n{}", font_face, GLOBAL_CSS);
    rsx! {
       style { "{global_css}"  }
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
                        .with_title("Casdesk 1.2.0")
                        .with_inner_size(LogicalSize::new(800.0, 800.0)),
                ),
        )
        .launch(App);
}
