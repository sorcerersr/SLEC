use std::time::Duration;

use dioxus::prelude::*;
use humantime::format_duration;

#[inline_props]
pub fn Exposure(cx: Scope, exposure_in_millis: u64) -> Element {
    let duration = Duration::from_millis(*exposure_in_millis);
    let formated_duration = format_duration(duration);
    cx.render(rsx! {
        div { h2 { "{formated_duration}" } }
    })
}
