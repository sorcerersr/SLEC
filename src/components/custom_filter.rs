use crate::{model::Filter, AppState};
use dioxus::prelude::*;
use rust_i18n::t;

pub fn CustomFilter(cx: Scope) -> Element {
    let _app_state = use_shared_state::<AppState>(cx).unwrap();
    let filters = use_ref(cx, || Filter::filter_list());
    cx.render(rsx!(
    article {
        h3 { t!("custom_filter") }

            button { onclick: move |_| {
                        let mut default_filters = Filter::default_filter_list();
                        Filter::store_filter_list(&default_filters);
                        filters.write().clear();
                        filters.write().append(&mut default_filters);
                    },
                    t!("filter.reset")
                    }
        (0..(&*filters.read()).len()).map(|index| {
            let filter = (&*filters.read()).get(index).unwrap().clone();
            rsx!(
                FilterComponent { filter: filter }
                rsx!(div { "align": "right",
                    button { onclick: move |_| {
                        filters.write().remove(index);
                        Filter::store_filter_list(&filters.read());

                    },
                    t!("filter.delete")
                    }
                })
            )}
        )}
    ))
}

#[component]
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

        }
    ))
}
