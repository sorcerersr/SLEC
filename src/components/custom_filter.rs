use crate::model::Filter;
use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn CustomFilter() -> Element {
    let mut filters = use_signal(Filter::filter_list);
    let custom_filter = t!("custom_filter");
    let add_filter = t!("filter.add");
    let reset_to_defaults = t!("filter.reset");

    let add_filter_handler = move |_| {
        let mut sig = filters.write();
        let id = Filter::next_id(&sig);
        let mut new_filter = Filter::new_custom(1, 0.0, "Custom".to_owned());
        new_filter.id = id;
        sig.push(new_filter);
        Filter::store_filter_list(&sig);
    };

    let reset_handler = move |_| {
        let defaults = Filter::reset_to_defaults();
        let mut sig = filters.write();
        *sig = defaults;
        Filter::store_filter_list(&sig);
    };

    let remove_callback = use_callback(move |id: usize| {
        let mut sig = filters.write();
        Filter::remove_filter(&mut sig, id);
        Filter::store_filter_list(&sig);
    });

    rsx!(
        article {
            h3 { "{custom_filter}" }
            for f in filters.read().iter() {
                FilterComponent {
                    key: "{f.id}",
                    filter: f.clone(),
                    on_remove: remove_callback
                }
            }
            button { onclick: add_filter_handler, "{add_filter}" }
            button { onclick: reset_handler, "{reset_to_defaults}" }
        }
    )
}

#[component]
fn FilterComponent(filter: Filter, on_remove: Callback<usize>) -> Element {
    let display_name = t!("filter.display_name");
    let fstop_reduction = t!("filter.fstop_reduction");
    let factor = t!("filter.factor");
    let delete = t!("filter.delete");
    rsx!(
        article {
            div { "class": "grid",
                label { "for": "display_name",
                    "{display_name}",
                    input {
                        value: &*filter.display_name,
                        "type": "text",
                        id: "display_name",
                        name: "display_name",
                        "required"
                    }
                }
                label { "for": "fstop_reduction",
                    "{fstop_reduction}",
                    input {
                        value: "{filter.fstop_reduction}",
                        "type": "number",
                        id: "fstop_reduction",
                        name: "fstop_reduction"
                    }
                }
                label { "for": "factor",
                    "{factor}",
                    input {
                        value: "{filter.factor}",
                        "type": "number",
                        id: "factor",
                        name: "factor"
                    }
                }
            }
            div { "align": "right",
                a { "href": "#", role: "button", onclick: move |e| { e.prevent_default(); on_remove.call(filter.id); }, "{delete}" }
            }
        }
    )
}
