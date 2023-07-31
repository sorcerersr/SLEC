use dioxus::prelude::*;

use crate::components::{BackButton, Exposure};

#[inline_props]
pub fn Timer(cx: Scope, exposure_in_millis: u64) -> Element {
    cx.render(rsx!(

        main { class: "container",
            BackButton {}
            Exposure { exposure_in_millis: *exposure_in_millis }
        }
    ))
}
