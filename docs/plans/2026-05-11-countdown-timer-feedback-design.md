# Countdown Timer Feedback Design

**Date:** 2026-05-11
**Status:** Validated

## Problem

The countdown timer in `src/timer.rs` counts down silently and simply `break`s when it reaches 0. The user has no feedback — no sound, no visual cue — and must keep watching the screen to know when the exposure is complete.

## Goals

- Provide clear, reliable feedback when the countdown reaches 0
- Support both active use (user holding shutter open) and passive use (camera on self-timer, user away)
- Be non-intrusive but hard to miss

## Design Decisions

### 1. Feedback Modalities: Sound + Visual

Both modalities are used so the feature works in all environments (silent mode, loud environment, etc.).

### 2. Sound: Single Beep at T=0

- **Type:** Single beep/chime at the end only
- **Frequency:** 800 Hz sine wave
- **Duration:** 300 ms
- **Implementation:** Web Audio API via `web-sys` — no audio files needed
- **Fallback:** If `AudioContext` creation fails (browser autoplay block), the visual pulse still works

### 3. Visual: Pulsing Exposure Display

The `Exposure` component's value alternates between the real countdown value and `0` every 500 ms.

**Phase 1 — Urgency pulse (last 10 seconds):**
- Countdown ≤ 10,000 ms: pulsing begins
- Creates a subtle flicker that draws attention

**Phase 2 — Full pulse at T=0:**
- Beep plays once
- Pulsing continues at 500 ms interval

**Phase 3 — Auto-reset (after 2 seconds):**
- All signals reset to initial state
- User can immediately tap "Start" again

### 4. Guard Against Double-Countdowns

If the user taps "Start" while a countdown is already running, the button does nothing. This fixes a pre-existing bug where multiple countdowns could run simultaneously.

## Architecture

### Files Changed

| File | Change |
|---|---|
| `src/timer.rs` | Add beep function, pulse logic, new signals |
| `Cargo.toml` | Add `AudioContext`, `OscillatorNode`, `AudioDestinationNode` to `web-sys` features |

### New State in `Timer` Component

```rust
let mut pulse_value = use_signal(|| exposure_in_millis); // value shown to user
let mut is_done = use_signal(|| false);                  // countdown finished flag
```

### New Function

```rust
fn play_beep() {
    // Creates AudioContext, generates 800Hz oscillator, plays for 300ms
}
```

### Data Flow

```
countdown() loop
  └─ if curexp > 10_000: pulse_value = curexp (no pulsing)
  └─ if 0 < curexp <= 10_000: spawn pulse task (toggle every 500ms)
  └─ if curexp == 0:
        → play_beep()
        → pulse task continues
        → spawn 2-second timer → reset all signals
```

### Pulse Task Logic

```rust
spawn(async move {
    loop {
        TimeoutFuture::new(500).await;
        if pulse_value() == curexp() {
            pulse_value.set(0);
        } else {
            pulse_value.set(curexp());
        }
    }
});
```

### Cargo.toml Change

Add to `web-sys` features:
```toml
"AudioContext", "OscillatorNode", "AudioDestinationNode"
```

## Edge Cases

| Scenario | Behavior |
|---|---|
| User navigates away during countdown | `spawn` task dropped on unmount — no sound plays |
| AudioContext blocked by browser | Silent failure; visual pulse still works |
| Very short exposure (<1 second) | Urgency pulse never triggers; beep still plays |
| User taps "Start" while running | Button is disabled (guard check) |
| Multiple rapid taps after reset | Allowed — each tap starts a fresh countdown |

## Testing

- Manual testing on desktop browser (Chrome/Firefox)
- Manual testing on mobile browser (Safari/Chrome)
- Verify beep plays on first user gesture (browser autoplay policy)
- Verify pulsing is smooth and doesn't cause layout shifts
- Verify auto-reset works correctly and new countdown starts cleanly
