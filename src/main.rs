#![allow(non_snake_case)]

rust_i18n::i18n!("locales", fallback = "en");
use dioxus::prelude::*;

mod components;
mod model;
use components::DarkModeToggle;

mod calculator;
use calculator::Calculator;

mod languages;

mod settings;
use languages::Language;
use settings::Settings;

// Entrypoint of the application
fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

// Enumeration to define the navigatable views
enum View {
    Calculator,
    Settings,
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
    pub fn toggle_view(&mut self) {
        match self.view {
            View::Calculator => self.view = View::Settings,
            View::Settings => self.view = View::Calculator,
        }
    }
}

fn app(cx: Scope) -> Element {
    // init i18n before doing anything else
    let language = languages::init();

    // init application wide state
    use_shared_state_provider(cx, || AppState {
        language,
        shutter_speed: 1.0 / 15.0,
        total_fstop_reduction: 0.0,
        view: View::Calculator,
    });

    cx.render(rsx! {

        Header {}
        Main {}
    })
}

// Header-Bar Component
// Visible for both views (calculator and settings)
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header { class: "container",
            nav {
                ul {
                    li {
                        strong { "📷 SLEC  " }
                        small { env!("CARGO_PKG_VERSION") }
                    }
                }
                ul {
                    li { DarkModeToggle {} }

                    li { GearLink {} }
                    li {}
                }
            }
        }
    })
}

// a clickable gear-icon to show/hide the settings view
fn GearLink(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    cx.render(rsx!(
        a { onclick: move |_| {
                app_state.write().toggle_view();
            }, href: "#", "⚙️" }
    ))
}

// The main component for the content
pub fn Main(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    cx.render(rsx!(
        #[allow(unused_parens)]
        // without those extra parens dioxus fmt will break the match statement
        (match app_state.read().view {
            View::Calculator => rsx!(Calculator {}),
            View::Settings => rsx!(Settings {}),
        })
    ))
}
