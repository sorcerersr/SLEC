use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::components::{BackButton, Exposure};

#[component]
pub fn Timer(exposure_in_millis: u64) -> Element {
    let mut curexp = use_signal(|| exposure_in_millis);
    let is_running = use_signal(|| false);

    let countdown = move |_| {
        spawn(async move {
            loop {
                TimeoutFuture::new(100).await;
                if curexp() >= 100 {
                    curexp -= 100;
                } else {
                    curexp.set(0);
                    break;
                }
            }
        });
    };

    rsx!(

        main { class: "container",
            BackButton {}
            Exposure { exposure_in_millis: curexp() }
            section {
                if !is_running() {
                    {rsx!(button { class:"outline",
                                  onclick: countdown,
                                 "Start"
                        })
                    }}
            }
            section {}
            BackButton {}
        }
    )
}
