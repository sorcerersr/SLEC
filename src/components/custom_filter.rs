use crate::{model::Filter, AppState};
use dioxus::prelude::*;
use rust_i18n::t;

pub fn CustomFilter(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let filters = use_ref(cx, || Filter::filter_list());
    cx.render(rsx!(
        article {
            h3 { t!("custom_filter") }
            for filter in &*filters.read() {
                FilterComponent { key: "{filter.id}", filter: filter.clone() }
            }
        }
    ))
}

#[inline_props]
fn FilterComponent(cx: Scope, filter: Filter) -> Element {
    cx.render(rsx!(
        article {
            div { "class": "grid",
                label { "for": "display_name",
                    t!("filter.display_name"),
                    input {
                        value: &*filter.display_name,
                        "type": "text",
                        id: "display_name",
                        name: "display_name",
                        "required"
                    }
                },

            }
        }
    ))
}
