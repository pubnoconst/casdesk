use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiChevronLeft;
use dioxus_free_icons::Icon;
use crate::Route;

#[derive(Clone, Copy)]
struct Item {
    id: u64,
    value: f64,
}

#[component]
pub fn Quote() -> Element {
    let nav = navigator();
    let mut items = use_signal(|| Vec::<Item>::new());
    let mut input = use_signal(|| String::new());

    rsx! {
        div {
            class: "primary-body",
            div {
                class: "primary-navigator",
                button {
                    class: "back-button",
                    onclick: move |_| {
                        nav.push(Route::Home {}); 
                    },
                    Icon {
                        icon: FiChevronLeft
                    }
                    "Back"
                }
                div {
                    h2 {
                        class: "title-text",
                        "Quote"
                    }
                }
            }
            div {
                id: "quote-body",
                div {
                    id: "quote-preview",
                    if items.is_empty() {
                        h3 {
                            class: "title-text",
                            "No parts added yet"
                        }
                    } else {
                        table {  
                            thead {
                                tr {
                                    th { "Price" }
                                    th { "Quote" }
                                    th { "Action" }
                                }
                            }
                            tbody {  
                                for item in items.read().iter() {
                                    tr {
                                        td { "{item.value}" }
                                        td {
                                            button {
                                                onclick: {
                                                    let id = item.id.clone();
                                                    move |_| {
                                                        items.write().retain(|x| x.id != id);                                                   
                                                    }
                                                },
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        id: "add-button-card",
                        h3 {
                            class: "title-text",
                            "Add item"
                        }
                        input {
                            r#type: "number",
                            value: "{input()}",
                            placeholder: "Part cost",
                            oninput: move |e| {
                                input.set(e.value());
                            }
                        }
                        button {
                            onclick: move |_| {
                                if let Ok(num) = input.clone().read().parse::<f64>() {
                                    let id = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
                                    items.write().push(Item { id: id, value: num });
                                }
                            },
                            "Confirm"
                        }
                    }
                    button {
                        class: "reset-btn",
                        onclick: move |_| {
                            items.set(Vec::new());
                        },
                        "Reset Table"
                    }
                    h3 {
                        id: "quote-total",
                        class: "title-text",
                        "Total: "
                    }
                }
            }
        }
    }
}