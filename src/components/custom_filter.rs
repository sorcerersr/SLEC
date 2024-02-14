use crate::{
    model::{Filter, Filters, FiltersAction},
    AppState,
};
use gloo_console::log;

use dioxus::prelude::*;
use rust_i18n::t;


pub fn CustomFilter(cx: Scope) -> Element {

    let _app_state = use_shared_state::<AppState>(cx).unwrap();
    let filters = use_ref(cx, || Filters::new());

    let onclick = move |event| {
        log!("within onclick callback!");
        filters.write().reduce(event);
    };


    cx.render(rsx!(
    article {
        h3 { t!("custom_filter") }
            div { "align": "right",
                button { 
                    width: "25%",
                    onclick: move |_| {
                    filters.write().reset();
                        },
                        t!("filter.reset")
                }
            }

            for filter in &*filters.read().list {
                FilterComponent{key: "{filter.id}", filter: filter.clone(), onclick: onclick}
              
            }
        }
    ))
    
}

#[derive(Props)]
pub struct FilterComponentProps<'a> {
    filter: Filter,
    onclick: EventHandler<'a, FiltersAction>,
}

fn FilterComponent<'a>(cx: Scope<'a, FilterComponentProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        article {
            div { "class": "grid",
                label { "for": "display_name",
                    t!("filter.display_name"),
                    input {
                        value: &*cx.props.filter.display_name,
                        "type": "text",
                        id: "display_name",
                        name: "display_name",
                        "required"
                    }
                }
                label { "for": "fstop_reduction",
                    t!("filter.fstop_reduction"),
                    input {
                        value: "{cx.props.filter.fstop_reduction}",
                        "type": "number",
                        id: "fstop_reduction",
                        name: "fstop_reduction"
                    }
                }
                label { "for": "factor",
                    t!("filter.factor"),
                    input {
                        value: "{cx.props.filter.factor}",
                        "type": "number",
                        id: "factor",
                        name: "factor"
                    }
                }
            }
            div { "align": "right",
                    button { 
                        width: "25%",
                        onclick: move |_| 
                        cx.props.onclick.call(FiltersAction::Remove(cx.props.filter.id)),
                    t!("filter.delete")
                }
            }
            
        }
    ))
}
