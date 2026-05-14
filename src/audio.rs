use std::cell::RefCell;
use web_sys::{AudioContext, GainNode, OscillatorType};

// Global AudioContext singleton — created on first user gesture, resumed immediately.
thread_local! {
    static AUDIO_CTX: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
}

/// Initialize the shared AudioContext. Call from a user-gesture context to resume.
pub fn init_audio() {
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
pub fn play_completion_sound() {
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
pub fn play_tick_beep() {
    with_audio(|audio_ctx| {
        let start_time = audio_ctx.current_time();
        let beep_duration = 0.05; // 50ms — short and subtle

        let Ok(oscillator) = audio_ctx.create_oscillator() else {
            return;
        };
        oscillator.set_type(OscillatorType::Sine);
        let frequency = oscillator.frequency();
        let _ = frequency.set_value_at_time(800.0, start_time); // 800Hz — higher than completion

        // Use GainNode for lower volume (0.25 vs 1.0 for completion)
        let Ok(gain_node) = GainNode::new(audio_ctx) else {
            return;
        };
        let gain = gain_node.gain();
        let _ = gain.set_value_at_time(0.25, start_time);

        let destination = audio_ctx.destination();
        let _ = oscillator.connect_with_audio_node(&gain_node);
        let _ = gain_node.connect_with_audio_node(&destination);
        let _ = oscillator.start_with_when(start_time);
        let _ = oscillator.stop_with_when(start_time + beep_duration);
    });
}
