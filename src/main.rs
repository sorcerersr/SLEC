#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components;
use components::DarkModeToggle;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {}
        Main {}
        Footer {}
    })
}

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header { class: "container",
            nav {
                ul {
                    li { strong { "📷 SLEC" } }
                }
                ul {
                    li { DarkModeToggle {} }

                    li { "⚙️" }
                    li {}
                }
            }
        }
    })
}

pub fn Main(cx: Scope) -> Element {
    cx.render(rsx! {

        main { class: "container",

            section { class: "section", ShutterSpeed {} }
            section { class: "section",
                div { class: "container", span { color: "is-white", "Hello, world!" } }
            }
            section { class: "section" }
        }
    })
}

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {

        footer { class: "container",
            section { class: "section has-dark-background", env!("CARGO_PKG_VERSION") }
        }
    })
}

pub fn ShutterSpeed(cx: Scope) -> Element {
    let mut shutter_index = 27;
    let label_text = format!("Shutter Speed = {}", shutter_index);
    cx.render(rsx! {
        label { "for": "shutter_speed",
            label_text,
            input {
                r#type: "range",
                oninput: move |event| { shutter_index = event.value.parse::<i64>().unwrap() },
                min: "0",
                max: "54",
                value: "{shutter_index}",
                id: "shutter_speed_slider",
                name: "shutter_speed"
            }
        }
    })
}
