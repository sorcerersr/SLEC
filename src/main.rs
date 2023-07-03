#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components;
use components::{DarkModeToggle, Slider};

mod model;
use model::{Filter, ShutterSpeed};

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

struct AppState {
    shutter_speed: f64,
    total_fstop_reduction: f64,
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || AppState {
        shutter_speed: 1.0 / 15.0,
        total_fstop_reduction: 0.0,
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

            section { class: "section",
                ShutterSpeed {}
                Filters {}
            }
            section { class: "section", FinalExposure {} }
            section { class: "section", "f-stop reduction: {app_state.read().total_fstop_reduction:}" }
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

pub fn Filters(cx: Scope) -> Element {
    let filters = use_ref(cx, || Filter::filter_array());
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let fstop_reduction = filters
        .read()
        .iter()
        .filter(|filter| filter.selected)
        .map(|filter| filter.fstop_reduction)
        .sum::<f64>();

    if app_state.read().total_fstop_reduction != fstop_reduction {
        app_state.write().total_fstop_reduction = fstop_reduction;
    }

    cx.render(rsx! {
        div { class: "grid",
            (0..filters.read().len()).map(|index| rsx!(
                        div{ margin:"10px 0px",
                                    input {

                                    oninput: move |event|
                                              filters.write().get_mut(index).unwrap().set_selected(event.value == "true"),
                                         "type":"checkbox", id:"filter_switch", name:"filter_switch", role:"switch"},
                                    filters.read().get(index).unwrap().display_name.clone(),
                                    }
            ))
        }
    })
}

pub fn FinalExposure(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let exposure_time =
        app_state.read().shutter_speed * app_state.read().total_fstop_reduction.exp2();
    cx.render(rsx! {"Time to expose: {exposure_time}"})
}
