use dioxus::prelude::*;

#[inline_props]
pub fn Slider<'a>(
    cx: Scope<'a>,
    min: usize,
    max: usize,
    value: usize,
    label: String,
    on_input: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {

        label { "for": "range",
            "Shutter Speed : {label}"
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
