use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::components::{BackButton, Exposure};

#[component]
pub fn Timer(cx: Scope, exposure_in_millis: u64) -> Element {
    let curexp = use_state(cx, || *exposure_in_millis);
    let is_running = use_state(cx, || false);

    let countdown = move |_| {
        cx.spawn({
            let mut curexp = curexp.to_owned();
            let is_running = is_running.to_owned();
            is_running.set(true);
            async move {
                loop {
                    TimeoutFuture::new(100).await;
                    if *curexp.current() >= 100 {
                        curexp -= 100;
                    } else {
                        curexp.set(0);
                        break;
                    }
                }
            }
        });
    };

    cx.render(rsx!(

        main { class: "container",
            BackButton {}
            Exposure { exposure_in_millis: *curexp.get() }
            section {
                if !*is_running.get() {
                        rsx!(button { class:"outline",
                                    onclick: countdown,
                                                    "Start"
                        })
                                    }
            }
            section {}
            BackButton {}
        }
    ))
}
