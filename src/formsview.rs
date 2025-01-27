use freya::prelude::*;
use crate::root::*;

#[component]
pub fn Formsview(route: Signal<Route>) -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            font_family: "Font1",
            rect {
                cross_align: "center",
                label {
                    font_size: "50",
                    "Forms"
                }
                Button {
                    onclick: move |_| route.set(Route::Home),
                    label { "Back" }
                }
            }
        }
    )
}