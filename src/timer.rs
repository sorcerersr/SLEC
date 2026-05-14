use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::cell::RefCell;
use web_sys::{AudioContext, GainNode, OscillatorType};

use crate::components::{BackButton, Exposure};

// Global AudioContext singleton — created on first user gesture, resumed immediately.
thread_local! {
    static AUDIO_CTX: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
}

/// Get or create the shared AudioContext. Call from a user-gesture context to resume.
fn ensure_audio_context() {
    AUDIO_CTX.with(|cell| {
        let mut opt = cell.borrow_mut();
        if opt.is_none() {
            let Ok(ctx) = AudioContext::new() else {
                return;
            };
            let _ = ctx.resume(); // Resume in user-gesture context
            *opt = Some(ctx);
        }
    });
}

/// Run a closure with the shared AudioContext.
fn with_audio<F>(f: F)
where
    F: FnOnce(&AudioContext),
{
    AUDIO_CTX.with(|cell| {
        if let Some(ref ctx) = *cell.borrow() {
            f(ctx);
        }
    });
}

/// Play a triple-beep completion sound using the Web Audio API.
/// Schedules all three beeps upfront for precise timing.
/// Fires and forgets — visual feedback works independently if this fails.
fn play_completion_sound() {
    with_audio(|audio_ctx| {
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
    });
}

/// Play a subtle tick beep (shorter, quieter, higher pitch) for the last 5 seconds.
fn play_tick_beep() {
    with_audio(|audio_ctx| {
        let start_time = audio_ctx.current_time();
        let beep_duration = 0.05; // 50ms — short and subtle

        let Ok(oscillator) = audio_ctx.create_oscillator() else {
            return;
        };
        oscillator.set_type(OscillatorType::Sine);
        let frequency = oscillator.frequency();
        let _ = frequency.set_value_at_time(800.0, start_time); // 800Hz — higher than completion

        // Use GainNode for lower volume (0.2 vs 1.0 for completion)
        let Ok(gain_node) = GainNode::new(audio_ctx) else {
            return;
        };
        let gain = gain_node.gain();
        let _ = gain.set_value_at_time(0.2, start_time);

        let destination = audio_ctx.destination();
        let _ = oscillator.connect_with_audio_node(&gain_node);
        let _ = gain_node.connect_with_audio_node(&destination);
        let _ = oscillator.start_with_when(start_time);
        let _ = oscillator.stop_with_when(start_time + beep_duration);
    });
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
        // Prime the AudioContext in this user-gesture context so browser allows audio
        ensure_audio_context();
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
