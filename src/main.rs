#![allow(non_snake_case)]

use dioxus::prelude::*;
mod components;
mod model;
use components::DarkModeToggle;

mod calculator;
use calculator::Calculator;

mod settings;
use settings::Settings;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

enum View {
    Calculator,
    Settings,
}

struct AppState {
    view: View,
    shutter_speed: f64,
    total_fstop_reduction: f64,
}

impl AppState {
    pub fn toggle_view(&mut self) {
        match self.view {
            View::Calculator => self.view = View::Settings,
            View::Settings => self.view = View::Calculator,
        }
    }
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || AppState {
        shutter_speed: 1.0 / 15.0,
        total_fstop_reduction: 0.0,
        view: View::Calculator,
    });

    cx.render(rsx! {

        Header {}
        Main {}
    })
}

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

fn GearLink(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    cx.render(rsx!(
        a { onclick: move |_| {
                app_state.write().toggle_view();
            }, href: "#", "⚙️" }
    ))
}

pub fn Main(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    cx.render(rsx!(match app_state.read().view {
        View::Calculator => rsx!(Calculator {}),
        View::Settings => rsx!(Settings {}),
    }))
}
