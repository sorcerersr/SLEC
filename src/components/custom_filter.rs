use crate::{model::Filter, AppState};
use dioxus::prelude::*;
use rust_i18n::t;

pub fn CustomFilter(cx: Scope) -> Element {
    let _app_state = use_shared_state::<AppState>(cx).unwrap();
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
    let _app_state = use_shared_state::<AppState>(cx).unwrap();
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
                }
                label { "for": "fstop_reduction",
                    t!("filter.fstop_reduction"),
                    input {
                        value: "{filter.fstop_reduction}",
                        "type": "number",
                        id: "fstop_reduction",
                        name: "fstop_reduction"
                    }
                }
                label { "for": "factor",
                    t!("filter.factor"),
                    input {
                        value: "{filter.factor}",
                        "type": "number",
                        id: "factor",
                        name: "factor"
                    }
                }
            }
            div { "align": "right",
                a { "href": "#", role: "button", t!("filter.delete") }
            }
        }
    ))
}
