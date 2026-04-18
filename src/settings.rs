use std::env;

use crate::{
    components::{BackButton, CustomFilter},
    languages::{self, Language},
    APP_STATE,
};
use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn Settings() -> Element {
    rsx!(
        main { class: "container",
            BackButton {}
            LanguageSelection {}
            CustomFilter {}
            About {}
            BackButton {}
        }
    )
}

#[component]
pub fn LanguageSelection() -> Element {
    let lang = t!("language");
    rsx!(
        article {
            h3 { "{lang}" }
            fieldset {
                section {
                    label { "for": "english",
                        input {
                            "type": "radio",
                            aria_placeholder: "english",
                            name: "language",
                            value: "english",
                            checked: APP_STATE.read().language == Language::English,
                            oninput: move |_| {
                                APP_STATE.write().language = Language::English;
                                languages::set_language(Language::English);
                            }
                        }
                        "English"
                    }
                    label { "for": "german",
                        input {
                            "type": "radio",
                            id: "german",
                            name: "language",
                            value: "german",
                            checked: APP_STATE.read().language == Language::German,
                            oninput: move |_| {
                                APP_STATE.write().language = Language::German;
                                languages::set_language(Language::German);
                            }
                        }
                        "Deutsch"
                    }
                }
            }
        }
    )
}

#[component]
pub fn About() -> Element {
    // app_state only used here for triggering an rerender when language settings are changed
    let about = t!("about");
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "?.?.?".to_string());
    rsx!(
        article {
            h3 { "{about}" }
            div { "SLEC - Sorcerers Long Exposure Calculator - {version}" }
            div { a { href: "https://github.com/sorcerersr/SLEC", "https://github.com/sorcerersr/SLEC" } }
        }
    )
}
