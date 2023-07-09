use crate::AppState;
use dioxus::prelude::*;
use rust_i18n::t;

use humantime::format_duration;

use std::time::Duration;

use crate::components::Slider;
use crate::model::{Filter, ShutterSpeed};

pub fn Calculator(cx: Scope) -> Element {
    cx.render(rsx! {
        main { class: "container",
            section { class: "section",
                ShutterSpeed {}
                Filters {}
            }
            section { class: "section", FinalExposure {} }
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
    let filters = use_ref(cx, || Filter::filter_list());
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
                        "type":"checkbox", id:"filter_switch", name:"filter_switch", role:"switch"
                    },
                    filters.read().get(index).unwrap().display_name.clone(),
                }
            ))
        }
    })
}

pub fn FinalExposure(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    // calculate the shutter speed for the final exposure
    let exposure_time =
        app_state.read().shutter_speed * app_state.read().total_fstop_reduction.exp2();
    // format the result to be more human friendly
    let exposure_time = if exposure_time > 30.0 {
        // above 30 sec truncate the miliseconds
        (exposure_time.trunc() * 1000.0) as u64
    } else {
        (exposure_time * 1000.0).trunc() as u64
    };
    let duration = Duration::from_millis(exposure_time);
    let formated_duration = format_duration(duration);
    cx.render(rsx! {

        div { t!("time_to_expose"), ": " }
        div { h2 { "{formated_duration}" } }
    })
}
