use std::time::{SystemTime, UNIX_EPOCH};

use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::{FiChevronLeft, FiTrash2};
use dioxus_free_icons::Icon;

#[derive(Clone, Copy)]
struct Item {
    id: u128,
    value: f64,
}

fn quote(part_cost: f64) -> f64 {
    if part_cost < 1. { return 150. }
    let over_200 = (part_cost >= 200.) as i32 as f64;
    let over_350 = (part_cost > 350.) as i32 as f64;

    part_cost + 140.0 + 40.0 * over_200 + 70.0 * over_350
}

#[component]
pub fn Quote() -> Element {
    let nav = navigator();
    let mut items = use_signal(|| Vec::<Item>::new());
    let mut input = use_signal(|| String::new());
    let sum = items.read().iter().map(|item| quote(item.value)).sum::<f64>();
    let sum_text = format!("Total: ${:.2}", sum);
    let deposit_text = format!("Minimum deposit: ${:.2}", sum/2.);

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
                        div {
                            id: "empty-table-card",
                            h2 {
                                id: "empty-table-banner",
                                class: "title-text",
                                "No parts added yet"
                            }
                        }
                    } else {
                        div{
                            id: "quote-table-container",
                            table {
                                id: "quote-table",
                                thead {
                                    tr {
                                        th { "#" }
                                        th { "Price" }
                                        th { "Quote" }
                                        th { "Action" }
                                    }
                                }
                                tbody {
                                    for (i, item) in items.read().iter().enumerate() {
                                        tr {
                                            td { "{i + 1}"}
                                            td { "{item.value}" }
                                            td { "{quote(item.value)}" }
                                            td {
                                                button {
                                                    id: "quote-item-delete-btn",
                                                    class: "danger-button",
                                                    onclick: {
                                                        let id = item.id.clone();
                                                        move |_| {
                                                            items.write().retain(|x| x.id != id);
                                                        }
                                                    },
                                                    Icon {
                                                        icon: FiTrash2
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        id: "add-item-card",
                        input {
                            r#type: "number",
                            value: "{input()}",
                            placeholder: "Part Cost",
                            oninput: move |e| {
                                input.set(e.value());
                            }
                        }
                        button {
                            class: "encouraged-button",
                            id: "part-confirm-button",
                            onclick: move |_| {
                                if let Ok(num) = input.clone().read().parse::<f64>() {
                                    let id = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
                                    items.write().push(Item { id: id, value: num });
                                }
                                input.set(String::new());
                            },
                            "Add"
                        }
                    }
                    div {
                        id: "quote-total-card",
                        if items.read().len() > 0 {
                            h3 {
                                id: "quote-total",
                                class: "title-text",                           
                                "{sum_text}" 
                            }
                            h4 {
                                // id: "quote-deposit",
                                class: "title-text",                           
                                "{deposit_text}"
                            }
                        }
                    }
                   div { 
                        id: "reset-button-card",
                        button {
                            id: "reset-button",
                            class: "danger-button",
                            onclick: move |_| {
                                items.set(Vec::new());
                            },
                            "Reset Table"
                        }
                    }
                   
                }
            }
        }
    }
}
