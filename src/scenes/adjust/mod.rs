use std::str::FromStr;
use dioxus::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const CSS: &str = include_str!("adjust.css");

#[component]
pub fn Adjust() -> Element {
    let mut software = use_signal(|| dec!(0));
    let mut machine = use_signal(|| dec!(0));

    let extra = use_memo(move || *machine.read() - *software.read());

    let missing_extra = use_memo(move || {
        (*extra.read() - *extra.read() * dec!(0.009))
            .round_dp(2)
    });
    let missing_extra_neg = use_memo(move || dec!(-1) * *missing_extra.read());

    rsx! {
        style { "{CSS}" }
        div {
            class: "frame",
            super::NavBar { page_title: "Adjust POS" }
            div {
                id: "content",
                div {
                    class: "card-container",
                    div {
                        class: "card",
                        "EFTPOS Total in Mybug"
                        input {
                            r#type: "number",
                            value: "{software}",
                            oninput: move |e| software.set(Decimal::from_str(&e.value()).unwrap_or(dec!(0)))
                        }
                    }
                    div {
                        class: "card",
                        "EFTPOS Machine Total" 
                        input {
                            r#type: "number",
                            value: "{machine}",
                            oninput: move |e| machine.set(Decimal::from_str(&e.value()).unwrap_or(dec!(0)))
                        }
                    }
                }
                div {
                    class: "summary",
                    if *software.read() > *machine.read() {
                        "You are missing ${missing_extra_neg} sales from the EFTPOS Machine. Mybug is behind ✔️"
                    } if *software.read() < *machine.read() {
                        "You recorded ${missing_extra} extraneous sales in Mybug. EFTPOS is behind ⚠️"
                    }
                }
            }
        }
    }    
}
