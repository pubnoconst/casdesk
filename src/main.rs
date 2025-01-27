#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use freya::prelude::*;
mod formsview;
mod root {
    use freya::prelude::*;
    pub static FONT1: &[u8] = include_bytes!("../resources/Jost-500-Medium.ttf");

    pub fn App() -> Element {
        let route = use_signal(|| Route::Home);
        match *route.clone().read() {
            Route::Home => rsx!(Homeview { route }),
            Route::Forms => rsx!(crate::formsview::Formsview { route }),
        }
    }

    #[derive(Clone, Copy)]
    pub enum Route {
        Home,
        Forms,
    }

    #[component]
    fn Homeview(route: Signal<Route>) -> Element {
        rsx!(
            rect{
                height: "100%",
                width: "100%",
                main_align: "center",
                cross_align: "center",
                background: "rgb(0, 119, 182)",
                color: "white",
                shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                font_family: "Font1",
                rect {
                    label {
                        font_size: "75",
                        font_weight: "bold",
                        "Casdesk."
                    }
                }
                // todo: add a <hr>
                rect {
                    direction: "horizontal",
                    margin: "10px",
                    Button {
                        onclick: move |_| route.set(Route::Forms),
                        label {
                                "Forms"
                        }
                    }
                    Button {
                        label {
                            "Calculator"
                        }
                    }
                    Button {
                        label {
                            "Quote"
                        }
                    }
                }
            }
        )
    }
}

fn main() {
    launch_cfg(
        root::App,
        LaunchConfig::<()>::builder()
            .with_title("Casdesk")
            // <a href="https://www.flaticon.com/free-icons/tax" title="tax icons">Tax icons created by Vectors Tank - Flaticon</a>
            // Todo fiture out wich winit version Freya uses
            // .with_icon("../resources/AppIcon.png")
            .with_font("Font1", root::FONT1)
            .with_width(640.)
            .with_height(500.)
            .build(),
    );
}
