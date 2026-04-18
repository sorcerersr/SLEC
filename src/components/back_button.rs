use dioxus::prelude::*;
use rust_i18n::t;

use crate::APP_STATE;

#[component]
pub fn BackButton() -> Element {
    let back = t!("back");
    rsx!(button {
        onclick: move |_| { APP_STATE.write().switch_view(crate::View::Calculator) },
        "{back}"
    })
}
