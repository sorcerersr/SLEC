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

    let edit_callback = use_callback(move |updated: Filter| {
        let mut sig = filters.write();
        if let Some(f) = sig.iter_mut().find(|f| f.id == updated.id) {
            *f = updated;
        }
        Filter::store_filter_list(&sig);
    });

    rsx!(
        article {
            h3 { "{custom_filter}" }
            for f in filters.read().iter() {
                FilterComponent {
                    key: "{f.id}",
                    filter: f.clone(),
                    on_remove: remove_callback,
                    on_edit: edit_callback
                }
            }
            button { onclick: add_filter_handler, "{add_filter}" }
            button { onclick: reset_handler, "{reset_to_defaults}" }
        }
    )
}

#[component]
fn FilterComponent(
    filter: Filter,
    on_remove: Callback<usize>,
    on_edit: Callback<Filter>,
) -> Element {
    let display_name = t!("filter.display_name");
    let fstop_reduction = t!("filter.fstop_reduction");
    let factor = t!("filter.factor");
    let delete = t!("filter.delete");

    let filter_id = filter.id;
    let filter_display = filter.display_name.clone();
    let filter_fstop = filter.fstop_reduction;
    let filter_factor = filter.factor;

    let filter_for_name = filter.clone();
    let filter_for_fstop = filter.clone();
    let filter_for_factor = filter.clone();

    rsx!(
        article {
            div { "class": "grid",
                label { "for": "display_name",
                    "{display_name}",
                    input {
                        "type": "text",
                        id: "display_name",
                        name: "display_name",
                        "required": true,
                        value: filter_display.as_str(),
                        oninput: move |event| {
                            let mut f = filter_for_name.clone();
                            f.display_name = event.value();
                            on_edit.call(f);
                        }
                    }
                }
                label { "for": "fstop_reduction",
                    "{fstop_reduction}",
                    input {
                        "type": "number",
                        id: "fstop_reduction",
                        name: "fstop_reduction",
                        value: "{filter_fstop}",
                        oninput: move |event| {
                            if let Ok(val) = event.value().parse::<f64>() {
                                let mut f = filter_for_fstop.clone();
                                f.fstop_reduction = val;
                                on_edit.call(f);
                            }
                        }
                    }
                }
                label { "for": "factor",
                    "{factor}",
                    input {
                        "type": "number",
                        id: "factor",
                        name: "factor",
                        value: "{filter_factor}",
                        oninput: move |event| {
                            if let Ok(val) = event.value().parse::<u64>() {
                                let mut f = filter_for_factor.clone();
                                f.factor = val;
                                on_edit.call(f);
                            }
                        }
                    }
                }
            }
            div { "align": "right",
                a { "href": "#", role: "button", onclick: move |e| { e.prevent_default(); on_remove.call(filter_id); }, "{delete}" }
            }
        }
    )
}
