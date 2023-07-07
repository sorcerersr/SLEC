use gloo_storage::{LocalStorage, Storage};

#[derive(PartialEq)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn locale(&self) -> String {
        match self {
            Language::English => "en".to_owned(),
            Language::German => "de-DE".to_owned(),
        }
    }

    pub fn for_locale_str(locale_str: &str) -> Language {
        if Language::German.locale() == locale_str {
            Language::German
        } else {
            Language::English
        }
    }
}

pub fn init() -> Language {
    if let Ok(locale) = LocalStorage::get::<String>("locale") {
        rust_i18n::set_locale(&locale);
        Language::for_locale_str(&locale)
    } else {
        rust_i18n::set_locale("en");
        Language::English
    }
}

pub fn set_language(language: Language) {
    // ToDo: proper errorhandling
    let _ = LocalStorage::set("locale", language.locale());
    rust_i18n::set_locale(&language.locale());
}
