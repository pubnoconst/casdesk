#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

use freya::prelude::*;

static FONT1: &[u8] = include_bytes!("../resources/Jost-500-Medium.ttf");

fn main() {    
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Casdesk")
            // <a href="https://www.flaticon.com/free-icons/tax" title="tax icons">Tax icons created by Vectors Tank - Flaticon</a>
            // Todo fiture out wich winit version Freya uses
            // .with_icon("../resources/AppIcon.png")
            .with_font("Font1", FONT1)
            .with_width(1260.)
            .with_height(900.)
            .build(),
    );
}

fn app() -> Element {
    let route = use_signal(|| Route::Home);
    match *route.clone().read() {
        Route::Home => rsx!(Homeview { route }),
        Route::Forms => rsx!(Formsview { route }),
        Route::Calculator => rsx!(Calculatorview { route }),
        Route::Quote => rsx!(Quoteview { route }),
    }
}

#[derive(Clone, Copy)]
enum Route {
    Home,
    Forms,
    Calculator,
    Quote,
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
            rect {
                label {
                    font_size: "75",
                    font_weight: "bold",
                    font_family: "Font1",
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
                        font_family: "Font1",
                            "Forms"
                    }
                }
                Button {
                    onclick: move |_| route.set(Route::Calculator),
                    label {
                        font_family: "Font1",
                        "Calculator"
                    }
                }
                Button {
                    onclick: move |_| route.set(Route::Quote),
                    label {
                        font_family: "Font1",
                        "Quote"
                    }
                }
            }
        }
    )
}

#[component]
fn Formsview(route: Signal<Route>) -> Element {
    let mut value = use_signal(String::new);

    rsx!(
        label {
            "Value: {value}"
        }
        Input {
            value: value.read(),
            onchange: move |e| {
                 value.set(e)
            }
        }
        Button {
            onclick: move |_| route.set(Route::Home),
            label { "Back" }
        }
    )
}

#[component]
fn Calculatorview(route: Signal<Route>) -> Element {
    rsx!(
        label {
            "Calculator View"
        }
        Button {
            onclick: move |_| route.set(Route::Home),
            label { "Back" }
        }
    )
}

#[component]
fn Quoteview(route: Signal<Route>) -> Element {
    rsx!(
        label {
            "Quote View"
        }
        Button {
            onclick: move |_| route.set(Route::Home),
            label { "Back" }
        }
    )
}
