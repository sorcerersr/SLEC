use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn Slider(
    min: usize,
    max: usize,
    value: usize,
    label: String,
    on_input: EventHandler<FormEvent>,
) -> Element {
    let label_text = format!("{} : {}", t!("shutterspeed"), label);
    rsx! {
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
    }
}
