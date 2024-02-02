use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn Slider<'a>(
    cx: Scope<'a>,
    min: usize,
    max: usize,
    value: usize,
    label: String,
    on_input: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let label_text = format!("{} : {}", t!("shutterspeed"), label);
    cx.render(rsx! {
        label { "for": "range",
            "{label_text}"
            input {
                "type": "range",
                min: "{min}",
                max: "{max}",
                value: "{value}",
                id: "range",
                name: "range",
                oninput: move |event| on_input.call(event)
            }
        }
    })
}
