use crate::{
    components::{BackButton, CustomFilter},
    languages::{self, Language},
    AppState,
};
use dioxus::prelude::*;
use rust_i18n::t;

pub fn Settings(cx: Scope) -> Element {
    cx.render(rsx!(
        main { class: "container",
            BackButton {}
            LanguageSelection {}
            CustomFilter {}
            About {}
            BackButton {}
        }
    ))
}

pub fn LanguageSelection(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    cx.render(rsx!(
        article {
            h3 { t!("language") }
            fieldset {
                section {
                    label { "for": "english",
                        input {
                            "type": "radio",
                            aria_placeholder: "english",
                            name: "language",
                            value: "english",
                            checked: app_state.read().language == Language::English,
                            oninput: move |_| {
                                app_state.write().language = Language::English;
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
                            checked: app_state.read().language == Language::German,
                            oninput: move |_| {
                                app_state.write().language = Language::German;
                                languages::set_language(Language::German);
                            }
                        }
                        "Deutsch"
                    }
                }
            }
        }
    ))
}

pub fn About(cx: Scope) -> Element {
    // app_state only used here for triggering an rerender when language settings are changed
    let _app_state = use_shared_state::<AppState>(cx).unwrap();
    cx.render(rsx!(
        article {
            h3 { t!("about") }
            div { "SLEC - Sorcerers Long Exposure Calculator ", env!("CARGO_PKG_VERSION") }
            div { a { href: "https://github.com/sorcerersr/SLEC", "https://github.com/sorcerersr/SLEC" } }
        }
    ))
}
