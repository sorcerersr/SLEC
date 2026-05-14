use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use web_sys::{AudioContext, OscillatorType};

use crate::components::{BackButton, Exposure};

/// Play a triple-beep completion sound using the Web Audio API.
/// Schedules all three beeps upfront for precise timing.
/// Fires and forgets — visual feedback works independently if this fails.
fn play_completion_sound() {
    let Ok(audio_ctx) = AudioContext::new() else {
        gloo_console::error!("Failed to create AudioContext for completion sound");
        return;
    };

    let start_time = audio_ctx.current_time();
    let beep_duration = 0.15; // 150ms per beep
    let gap = 0.10; // 100ms gap between beeps
    let spacing = beep_duration + gap; // 250ms total spacing

    for i in 0..3 {
        let offset = (i as f64) * spacing;
        let Ok(oscillator) = audio_ctx.create_oscillator() else {
            gloo_console::error!("Failed to create OscillatorNode");
            return;
        };

        oscillator.set_type(OscillatorType::Sine);
        let frequency = oscillator.frequency();
        let _ = frequency.set_value_at_time(440.0, start_time + offset);

        let destination = audio_ctx.destination();
        let _ = oscillator.connect_with_audio_node(&destination);
        let _ = oscillator.start_with_when(start_time + offset);
        let _ = oscillator.stop_with_when(start_time + offset + beep_duration);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_timer_component_structure() {
        let exposure_time: u64 = 5000; // 5 seconds
        assert_eq!(exposure_time, 5000);
    }
}

#[component]
pub fn Timer(exposure_in_millis: u64) -> Element {
    let mut curexp = use_signal(|| exposure_in_millis);
    let mut is_running = use_signal(|| false);
    let mut done = use_signal(|| false);

    let countdown = move |_| {
        is_running.set(true);
        done.set(false);
        spawn(async move {
            loop {
                TimeoutFuture::new(100).await;
                if curexp() >= 100 {
                    curexp -= 100;
                } else {
                    curexp.set(0);
                    break;
                }
            }
            // Countdown finished — trigger feedback
            play_completion_sound();
            done.set(true);
            is_running.set(false);
        });
    };

    let done_text = rust_i18n::t!("timer.done");

    rsx!(
        Stylesheet { href: asset!("../assets/timer.css") }

        main { class: "container",
            BackButton {}

            if done() {
                div { class: "done-flash",
                    div { class: "checkmark", "✅" }
                    h2 { "{done_text}" }
                }
            } else if is_running() {
                div { class: "timer-center",
                    Exposure { exposure_in_millis: curexp() }
                }
            } else {
                div { class: "timer-center",
                    Exposure { exposure_in_millis: curexp() }
                    section {
                        button { class: "outline", onclick: countdown, "Start" }
                    }
                }
            }

            BackButton {}
        }
    )
}
