use dioxus::prelude::*;
use dioxus::desktop::use_window;

use crate::shared::icons::APP_LOGO;
use crate::shared::components::buttons::WindowControlButtons;


#[component]
pub fn Titlebar() -> Element {
    let window = use_window();
    let app_version = env!("CARGO_PKG_VERSION");

    rsx! {
			div {
				class: "titlebar",
				onmousedown: move |_| {
				    window.drag();
				},

				div { class: "titlebar__left",
					div {
						class: "titlebar__logo",
						dangerous_inner_html: APP_LOGO,
					}
					div { class: "titlebar__title", "Lampadion — {app_version}" }
				}

				div { class: "titlebar__right", WindowControlButtons {} }
			}
		}
}
