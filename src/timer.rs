use dioxus::prelude::*;

use crate::components::BackButton;
pub fn Timer(cx: Scope) -> Element {
    cx.render(rsx!(

        main { class: "container",
       BackButton {}, "timer view"
    }))
}
