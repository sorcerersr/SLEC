use crate::AppState;
use dioxus::prelude::*;

pub fn Settings(cx: Scope) -> Element {
    cx.render(rsx!(
        main { class: "container", section { class: "section", "settings...." } }
    ))
}
