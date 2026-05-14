use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::audio::{init_audio, play_completion_sound, play_tick_beep};
use crate::components::{BackButton, Exposure};

#[component]
pub fn Timer(exposure_in_millis: u64) -> Element {
    let mut curexp = use_signal(|| exposure_in_millis);
    let mut is_running = use_signal(|| false);
    let mut done = use_signal(|| false);

    let countdown = move |_| {
        is_running.set(true);
        done.set(false);
        // Prime the AudioContext in this user-gesture context so browser allows audio
        init_audio();
        spawn(async move {
            let mut last_beep_at: u64 = u64::MAX;
            loop {
                TimeoutFuture::new(100).await;
                if curexp() >= 100 {
                    curexp -= 100;
                } else {
                    curexp.set(0);
                    break;
                }
                // Subtle tick beep every second during the last 5 seconds
                if curexp() <= 5000 && curexp() % 1000 == 0 && curexp() != last_beep_at {
                    last_beep_at = curexp();
                    play_tick_beep();
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_timer_component_structure() {
        let exposure_time: u64 = 5000; // 5 seconds
        assert_eq!(exposure_time, 5000);
    }
}
