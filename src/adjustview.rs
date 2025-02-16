use std::str::FromStr;

use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::{
    FiAlertTriangle, FiCheck, FiCheckCircle, FiChevronLeft, FiCreditCard, FiFrown, FiMonitor, FiSmile
};
use dioxus_free_icons::Icon;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[component]
pub fn Adjust() -> Element {
    let nav = navigator();

    let mut mybug = use_signal(|| dec!(0));
    let mut eftpos = use_signal(|| dec!(0));

    let extra = use_memo(move || *eftpos.read() - *mybug.read());

    let missing_extra = use_memo(move || {
        (*extra.read() - *extra.read() * dec!(0.009))
            .round_dp(2)
    });

    let missing_lost = use_memo(move || dec!(-1) * *missing_extra.read());

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
                        "POS Adjustment"
                    }
                }
            }
            div {
                id: "adjust-body",
                div {
                    id: "adjust-cards",
                    div {
                        class: "card",
                        Icon {
                            icon: FiMonitor
                        }
                        h4 {
                            "Eftpos total in Mybug"
                        }
                        div {
                            class: "card-row",
                            "$ "
                            input {
                                r#type: "number",
                                oninput: move |e| mybug.set(Decimal::from_str(&e.value()).unwrap_or(dec!(0)))
                            }
                        }
                    }
                    div {
                        class: "card",
                        Icon {
                            icon: FiCreditCard
                        }
                        h4 {
                            "Eftpos total in the EFTPOS machine"
                        }
                        div {
                            class: "card-row",
                            "$ "
                            input {
                                r#type: "number",
                                oninput: move |e| eftpos.set(Decimal::from_str(&e.value()).unwrap_or(dec!(0)))
                            }
                        }
                    }
                }
            }
            div {
                id: "adjust-result",
                if *eftpos.read() > *mybug.read() {
                    div { "You are missing ${missing_extra} sale from the EFTPOS" }
                    Icon {
                        icon: FiCheckCircle
                    }
                } else if *eftpos.read() < *mybug.read() {
                    div { "You have ${missing_lost} extraneous sale in mybug" }
                    Icon {
                        icon: FiAlertTriangle
                    }
                }
            }
        }
    }
}
