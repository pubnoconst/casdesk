use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use arboard::Clipboard;
use dioxus::prelude::*;
use notify_rust::Notification;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const CSS: &str = include_str!("quote.css");

#[derive(Clone)]
struct QuoteEntry {
    id: u128,
    value: Decimal,
}

#[component]
pub fn Quote() -> Element {
    let mut quotes = use_signal(Vec::<QuoteEntry>::new);
    let total: Memo<Decimal> = use_memo(move || quotes.read().iter().map(|q| q.value).sum());
    let deposit: Memo<Decimal> = use_memo(move || *total.read() / Decimal::TWO);

    rsx! {
        style { "{CSS}" }
        div {
            class: "frame",
            super::NavBar { page_title: "Quote" }
            // Direct quote form
            div {
                id: "direct-form-container",
                form {
                    onsubmit: move |e| {
                        if let Some(v) = e.data().values().get("direct_price") {
                            if let Some(v) = v.first() {
                                if let Ok(v) = Decimal::from_str(v) {
                                    let id = SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis();
                                    quotes.write().push(QuoteEntry { id, value: v });
                                }
                            }
                        }
                    },
                    label { "Direct quote" }
                    div {
                        class: "input-row",
                        input {
                            r#type: "number",
                            name: "direct_price",
                            placeholder: "Amount will be added as-is"
                        }
                        button {
                            class: "encouraged-button",
                            r#type: "submit",
                            "Add"
                        }
                    }
                }
            }
            // Calculated quote form
            div {
                id: "quoted-form-container",
                form {
                    onsubmit: move |e| {
                        if let Some(v) = e.data().values().get("quotable_price") {
                            if let Some(v) = v.first() {
                                if let Ok(v) = Decimal::from_str(v) {
                                    let calculated = if v < dec!(200) {
                                        v + dec!(140)
                                    } else if v <= dec!(350) {
                                        v + dec!(180)
                                    } else {
                                        v + dec!(250)
                                    };
                                    let id = SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis();
                                    quotes.write().push(QuoteEntry { id, value: calculated });
                                }
                            }
                        }
                    },
                    label { "Calculated quote" }
                    div {
                        class: "input-row",
                        input {
                            r#type: "number",
                            name: "quotable_price",
                            placeholder: "Quotation formula will be applied"
                        }
                        button {
                            class: "encouraged-button",
                            r#type: "submit",
                            "Add"
                        }
                    }
                }
            }
            // Quote summary and reset button
            div {
                id: "quote-summary",
                div {
                    class: "summary-amounts",
                    div {
                        class: "summary-row",
                        span { class: "label", "Total quote:" }
                        span { class: "value", "${total}" }
                        button { 
                            onclick: move |_| {
                                if let Ok(mut cb) = Clipboard::new() {
                                    if cb.set_text(format!("{:.2}", total.read())).is_ok() {
                                        let _ = Notification::new()
                                            .body(&format!("Copied {:.2} to clipboard", total.read()))
                                            .appname("Casdesk")
                                            .show();
                                    }
                                }
                            },
                            "Copy total" 
                        }
                    }
                    div {
                        class: "summary-row",
                        span { class: "label", "Minimum deposit:" }
                        span { class: "value", "${deposit}" }
                        button { 
                            onclick: move |_| {
                                if let Ok(mut cb) = Clipboard::new() {
                                    if cb.set_text(format!("{:.2}", deposit.read())).is_ok() {
                                        let _ = Notification::new()
                                            .body(&format!("Copied {:.2} to clipboard", deposit.read()))
                                            .appname("Casdesk")
                                            .show();
                                    }
                                }
                            },
                            "Copy deposit" 
                        }
                    }
                }
                div {
                    id: "reset-button-container",
                    button {
                        onclick: move |_| {
                            quotes.write().clear();
                        },
                        class: "danger-button",
                        "Reset Table"
                    }
                }
            }
            // Quotes table with delete buttons
            div {
                id: "quotes-table",
                if quotes.read().len() > 0 {
                    table {
                        thead {
                            tr {
                                th { "Quote" }
                                th { "Action" }
                            }
                        }
                        tbody {
                            for quote in quotes.read().iter().cloned() {
                                tr {
                                    td { "{quote.value}" }
                                    td {
                                        button {
                                            onclick: move |_| {
                                                quotes.write().retain(|q| q.id != quote.id);
                                            },
                                            class: "danger-button",
                                            "Remove"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
