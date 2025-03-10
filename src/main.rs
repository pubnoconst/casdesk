#![windows_subsystem = "windows"]

use dioxus::{desktop::LogicalSize, prelude::*};
mod scenes;
use scenes::{adjust::*, forms::*, home::*, quote::*, GLOBAL_CSS};
mod update_manager;
mod logger;
mod semver;

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
    let figtree_base64 = scenes::FIGTREE_FONT_BYTES;
    let font_face = format!(
        "@font-face {{ font-family: 'Figtree'; src: url('data:font/ttf;base64,{figtree_base64}') format('truetype'); }}"
    );
    let global_css = format!("{}\n{}", font_face, GLOBAL_CSS);
    rsx! {
       style { "{global_css}"  }
       Router::<Route> {}
    }
}

fn main() {
    if let Err(e) = logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }
    update_manager::update();
    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title(format!("Casdesk {}", env!("CARGO_PKG_VERSION")))
                        .with_inner_size(LogicalSize::new(800.0, 800.0)),
                ),
        )
        .launch(App);
}
