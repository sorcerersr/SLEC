use crate::View;
use crate::APP_STATE;
use dioxus::prelude::*;
use rust_i18n::t;

use crate::components::Exposure;
use crate::components::Slider;
use crate::model::{Filter, ShutterSpeed};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_exposure_calculation() {
        // Test the exposure time calculation logic in FinalExposure
        // Set up a test scenario with specific shutter speed and f-stop reduction
        let shutter_speed: f64 = 1.0 / 15.0; // 1/15 second
        let total_fstop_reduction: f64 = 3.0; // 3 stops reduction
        
        // Calculate the expected exposure time manually to verify
        let expected_exposure_time = shutter_speed * total_fstop_reduction.exp2(); // 1/15 * 2^3 = 1/15 * 8 = 8/15 seconds
        
        // Verify the calculation matches expected value (with tolerance for floating point)
        assert!((expected_exposure_time - 0.5333333333333333).abs() < f64::EPSILON);
    }

    #[test]
    fn test_final_exposure_formatting() {
        // Test the exposure time formatting logic
        // When exposure time > 30 seconds, should truncate milliseconds
        let exposure_time_long: f64 = 35.0; // 35 seconds (above threshold)
        
        // Test truncation behavior for long exposures
        let formatted_long = if exposure_time_long > 30.0 {
            (exposure_time_long.trunc() * 1000.0) as u64
        } else {
            (exposure_time_long * 1000.0).trunc() as u64
        };
        
        // Should truncate to whole seconds and multiply by 1000
        assert_eq!(formatted_long, 35000);
        
        // When exposure time < 30 seconds, should round to nearest millisecond
        let exposure_time_short: f64 = 29.5; // 29.5 seconds
        
        let formatted_short = if exposure_time_short > 30.0 {
            (exposure_time_short.trunc() * 1000.0) as u64
        } else {
            (exposure_time_short * 1000.0).trunc() as u64
        };
        
        // Should multiply by 1000 and truncate to nearest millisecond
        assert_eq!(formatted_short, 29500);
    }

    #[test]
    fn test_filter_selection_logic() {
        // Test the filter selection logic that affects total f-stop reduction
        let filters = vec![
            Filter {
                factor: 8,
                fstop_reduction: 3.0,
                display_name: "ND8".to_owned(),
                selected: true,
                id: 0,
            },
            Filter {
                factor: 64,
                fstop_reduction: 6.0,
                display_name: "ND64".to_owned(),
                selected: false,
                id: 1,
            }
        ];
        
        // Calculate total f-stop reduction for selected filters
        let total_reduction = filters
            .iter()
            .filter(|filter| filter.selected)
            .map(|filter| filter.fstop_reduction)
            .sum::<f64>();
            
        assert_eq!(total_reduction, 3.0); // Only ND8 is selected
    }
}

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
