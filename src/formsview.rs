use freya::prelude::*;
use crate::root::*;

#[component]
pub fn Formsview(route: Signal<Route>) -> Element {
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