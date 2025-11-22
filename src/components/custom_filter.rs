use crate::model::Filter;
use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn CustomFilter() -> Element {
    let filters = use_signal(|| Filter::filter_list());
    let custom_filter = t!("custom_filter");
    rsx!(
        article {
            h3 { "{custom_filter}" }
            for filter in filters.read().iter() {
                FilterComponent { key: "{filter.id}", filter: filter.clone() }
            }
        }
    )
}

#[component]
fn FilterComponent(filter: Filter) -> Element {
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
                a { "href": "#", role: "button", "{delete}" }
            }
        }
    )
}
