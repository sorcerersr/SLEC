use crate::AppState;
use dioxus::prelude::*;
use rust_i18n::t;

pub fn CustomFilter(cx: Scope) -> Element {
    let _app_state = use_shared_state::<AppState>(cx).unwrap();
    cx.render(rsx!(
        article {
            h3 { t!("custom_filter") }
        }
    ))
}
