#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components;
use components::{DarkModeToggle, Slider};

mod model;
use model::ShutterSpeed;

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
    let shutter_speeds = ShutterSpeed::shutter_speed_array();
    let index_of_selected_shutter_speed = use_state(cx, || 26);
    let label = shutter_speeds[*index_of_selected_shutter_speed.get()]
        .display_text
        .clone();
    cx.render(rsx! {
        Slider {
            min: 0,
            max: shutter_speeds.len() - 1,
            value: *index_of_selected_shutter_speed.get(),
            label: label,
            on_input: move |event: FormEvent| {
                index_of_selected_shutter_speed.set(event.value.parse::<usize>().unwrap());
            }
        }
    })
}
