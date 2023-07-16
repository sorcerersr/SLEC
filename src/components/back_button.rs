use dioxus::prelude::*;
use rust_i18n::t;

use crate::AppState;
pub fn BackButton(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    cx.render(rsx!(
        button { onclick: move |_| { app_state.write().switch_view(crate::View::Calculator) }, t!("back") }
    ))
}
