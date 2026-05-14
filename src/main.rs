#![allow(non_snake_case)]

rust_i18n::i18n!("locales", fallback = "en");

use dioxus::prelude::*;

mod components;
mod model;
use components::DarkModeToggle;

mod audio;

mod timer;
use timer::Timer;

mod calculator;
use calculator::Calculator;

mod languages;

mod settings;
use languages::Language;
use settings::Settings;

// Entrypoint of the application
fn main() {
    // launch the web app
    launch(app);
}

// Enumeration to define the navigatable views
#[derive(Clone, Copy)]
enum View {
    Calculator,
    Settings,
    Timer(u64),
}

// struct representing a globale applocation wide state
struct AppState {
    language: Language,
    view: View,
    shutter_speed: f64, // ToDo: move to prop or local state in Calculator view
    total_fstop_reduction: f64, // ToDo: move to prop or local state in Calculator view}
}

impl AppState {
    // Method for switching between the two views
    pub fn switch_view(&mut self, view: View) {
        self.view = view;
    }
}

// init application wide state
static APP_STATE: GlobalSignal<AppState> = Signal::global(|| AppState {
    language: languages::init(),
    shutter_speed: 1.0 / 15.0,
    total_fstop_reduction: 0.0,
    view: View::Calculator,
});

#[component]
fn app() -> Element {
    rsx! {
        Header {}
        Main {}
    }
}

// Header-Bar Component
// Visible for both views (calculator and settings)
pub fn Header() -> Element {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?");
    rsx! {
        header { class: "container",
            nav {
                ul {
                    li {
                        strong { "📷 SLEC  " }
                        small { "{version}" }
                    }
                }
                ul {
                    li { DarkModeToggle {} }

                    li { GearLink {} }
                    li {}
                }
            }
        }
    }
}

// a clickable gear-icon to show/hide the settings view
#[component]
fn GearLink() -> Element {
    let new_view = match APP_STATE.read().view {
        View::Calculator => View::Settings,
        View::Settings => View::Calculator,
        View::Timer(_) => View::Settings,
    };
    rsx!(
        a {
            onclick: move |_| {
                APP_STATE.write().switch_view(new_view);
            },
            href: "#",
            "⚙️"
        }
    )
}

// The main component for the content
#[component]
pub fn Main() -> Element {
    rsx!(
        Stylesheet { href: asset!("../assets/pico.min.css") }
        // without those extra parens dioxus fmt will break the match statement
        {
            match APP_STATE.read().view {
                View::Calculator => rsx!(Calculator {}),
                View::Settings => rsx!(Settings {}),
                View::Timer(exposure_time) => rsx!(Timer {
                    exposure_in_millis: exposure_time
                }),
            }
        }
    )
}
