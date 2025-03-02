use dioxus::prelude::*;
mod io;
mod sale;
mod purchase;
mod lease;
mod risk;

#[derive(PartialEq, Clone, Copy)]
enum FormState {
    RefurbsishedSale,
    NewSale,
    Purchase,
    Lease,
    BackGlass,
    FragileScreen,
}

const CSS :&str  = include_str!("forms.css");

#[component]
pub fn Forms() -> Element {
    let mut fs = use_signal(|| FormState::RefurbsishedSale);
    rsx! {
        style { "{CSS}" }
        div {
            class: "frame",
            super::NavBar { page_title: "Forms" }
            div {
                id: "tab",
                div {
                    id: "tabstrip",
                    if *fs.read() == FormState::RefurbsishedSale {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::RefurbsishedSale);
                            },
                            class: "active-tab",
                            "Refurbished device sale",
                        }                                                   
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::RefurbsishedSale);
                            },
                            "Refurbished device sale"
                        }
                    }
                    if *fs.read() == FormState::NewSale {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::NewSale);
                            },
                            class: "active-tab",
                            "New device sale",
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::NewSale);
                            },
                            "New device sale",
                        }
                    }
                    if *fs.read() == FormState::Purchase {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::Purchase);
                            },
                            class: "active-tab",
                            "Purchase",
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::Purchase);
                            },
                            "Purchase"  
                        }
                    }
                    if *fs.read() == FormState::Lease {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::Lease);
                            },
                            class: "active-tab",
                            "Lease",
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::Lease);
                            },          
                            "Lease"
                        }
                    }
                    if *fs.read() == FormState::BackGlass {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::BackGlass);
                            },
                            class: "active-tab",
                            "Back glass",
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::BackGlass);
                            },
                            "Back glass"
                        }
                    }
                    if *fs.read() == FormState::FragileScreen {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::FragileScreen);
                            },
                            class: "active-tab",
                            "Fragile screen",
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                fs.set(FormState::FragileScreen);
                            },
                            "Fragile screen"
                        }
                    } 
                }
                div {
                    id: "tabcontent",
                    if *fs.read() == FormState::RefurbsishedSale {
                        sale::Sale {  kind: sale::SaleType::Refurbished } 
                    } else if *fs.read() == FormState::NewSale {
                        sale::Sale { kind: sale::SaleType::New }
                    } else if *fs.read() == FormState::Purchase {
                        purchase::Purchase {}
                    } else if *fs.read() == FormState::Lease {
                        lease::Lease {}
                    } else if *fs.read() == FormState::BackGlass {
                        risk::RiskForm { kind: risk::RiskFormType::BackGlass }
                    } else if *fs.read() == FormState::FragileScreen {
                        risk::RiskForm { kind: risk::RiskFormType::FragileScreen }
                    }
                }
            }
        }
    }
}