use dioxus::prelude::*;

fn isDarkModePrefered() -> bool {
    let window = web_sys::window().unwrap();
    window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
}

fn setDarkModePreference(theme: &str) {
    let window = web_sys::window().unwrap();
    window
        .document()
        .unwrap()
        .document_element()
        .unwrap()
        .set_attribute("data-theme", theme);
}

enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn html_value(&self) -> String {
        match self {
            Theme::Dark => "dark".to_owned(),
            Theme::Light => "light".to_owned(),
        }
    }

    pub fn is_darkmode_checked(&self) -> String {
        match self {
            Theme::Dark => "true".to_owned(),
            Theme::Light => "false".to_owned(),
        }
    }
}

impl From<bool> for Theme {
    fn from(value: bool) -> Self {
        if value {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

pub fn DarkModeToggle(cx: Scope) -> Element {
    let theme = Theme::from(isDarkModePrefered());
    cx.render(rsx! {
        "🌞"
        input {
            oninput: move |event| {
                setDarkModePreference(&Theme::from(event.value == "true").html_value());
            },
            checked: "{theme.is_darkmode_checked()}",
            r#type: "checkbox",
            id: "theme_switch",
            name: "theme-switch",
            role: "switch"
        }
        "🌘"
    })
}
