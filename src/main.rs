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

struct AppState {
    shutter_speed: f64,
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || AppState {
        shutter_speed: 1.0 / 15.0,
    });
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
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    cx.render(rsx! {

        main { class: "container",

            section { class: "section", ShutterSpeed {} }
            section { class: "section",
                div { class: "container", span { color: "is-white", "Hello, world!" } }
            }
            section { class: "section",
                "Time to expose: {app_state.read().shutter_speed}"
                 }
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
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let shutter_speeds = ShutterSpeed::shutter_speed_array();
    let index_of_selected_shutter_speed = use_state(cx, || 26);
    let shutter_speed = &shutter_speeds[*index_of_selected_shutter_speed.get()];
    let label = shutter_speed.display_text.clone();
    cx.render(rsx! {
        Slider {
            min: 0,
            max: shutter_speeds.len() - 1,
            value: *index_of_selected_shutter_speed.get(),
            label: label,
            on_input: move |event: FormEvent| {
                let index = event.value.parse::<usize>().unwrap();
                index_of_selected_shutter_speed.set(index);
                let shutter_speed = &shutter_speeds[index];
                app_state.write().shutter_speed = shutter_speed.speed_value;
            }
        }
    })
}
