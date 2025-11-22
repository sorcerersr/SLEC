use crate::View;
use crate::APP_STATE;
use dioxus::prelude::*;
use rust_i18n::t;

use crate::components::Exposure;
use crate::components::Slider;
use crate::model::{Filter, ShutterSpeed};

#[component]
pub fn Calculator() -> Element {
    rsx! {
        main { class: "container",
            section { class: "section",
                ShutterSpeedComponent {}
                Filters {}
            }
            section { class: "section", FinalExposure {} }
        }
    }
}

#[component]
pub fn TimerButton(exposure_in_millis: u64) -> Element {
    rsx!(
        button { onclick: move |_| { APP_STATE.write().switch_view(View::Timer(exposure_in_millis)) },
            "Timer"
        }
    )
}

#[component]
pub fn ShutterSpeedComponent() -> Element {
    let shutter_speeds = ShutterSpeed::shutter_speed_array();
    let mut index_of_selected_shutter_speed = use_signal(|| 26);
    let shutter_speed = &shutter_speeds[*index_of_selected_shutter_speed.read()];
    let label = shutter_speed.display_text.clone();
    rsx! {
        Slider {
            min: 0,
            max: shutter_speeds.len() - 1,
            value: *index_of_selected_shutter_speed.read(),
            label: label,
            on_input: move |event: FormEvent| {
                let index = event.value().parse::<usize>().unwrap();
                index_of_selected_shutter_speed.set(index);
                let shutter_speed = &shutter_speeds[index];
                APP_STATE.write().shutter_speed = shutter_speed.speed_value;
            }
        }
    }
}

#[component]
pub fn Filters() -> Element {
    let mut filters = use_signal(|| Filter::filter_list());
    let fstop_reduction = filters
        .read()
        .iter()
        .filter(|filter| filter.selected)
        .map(|filter| filter.fstop_reduction)
        .sum::<f64>();

    if APP_STATE.read().total_fstop_reduction != fstop_reduction {
        APP_STATE.write().total_fstop_reduction = fstop_reduction;
    }

    rsx! {
        div { class: "grid",
            {(0..filters.read().len()).map(|index| rsx!(
                div{ margin:"10px 0px",
                    input {
                        oninput: move |event|
                            filters.write().get_mut(index).unwrap().set_selected(event.value() == "true"),
                        "type":"checkbox", id:"filter_switch", name:"filter_switch", role:"switch"
                    },
                    {filters.read().get(index).unwrap().display_name.clone()},
                }
            ))}
        }
    }
}

#[component]
pub fn FinalExposure() -> Element {
    // calculate the shutter speed for the final exposure
    let exposure_time =
        APP_STATE.read().shutter_speed * APP_STATE.read().total_fstop_reduction.exp2();
    // format the result to be more human friendly
    let exposure_time = if exposure_time > 30.0 {
        // above 30 sec truncate the miliseconds
        (exposure_time.trunc() * 1000.0) as u64
    } else {
        (exposure_time * 1000.0).trunc() as u64
    };
    let time_to_expose = t!("time_to_expose");
    rsx! {

        div { "{time_to_expose} : " }
        div { Exposure { exposure_in_millis: exposure_time } }
        div { TimerButton { exposure_in_millis: exposure_time } }
    }
}
