use dioxus::prelude::*;
use dioxus::desktop::use_window;

const APP_LOGO: Asset = asset!("/assets/images/lampadion.svg");

#[component]
pub fn Titlebar() -> Element {
    let window = use_window();
    let app_version = env!("CARGO_PKG_VERSION");

    let w_drag = window.clone();
    let w_min = window.clone();
    let w_max = window.clone();
    let w_close = window.clone();

    rsx! {
			div {
				class: "titlebar",
				onmousedown: move |_| {
				    w_drag.drag();
				},

				div { class: "titlebar__left",
					img {
						class: "titlebar__logo",
						src: APP_LOGO,
						alt: "Lampadion Logo",
					}
					div { class: "titlebar__title", "Lampadion — {app_version}" }
				}

				div { class: "titlebar__right",
					div { class: "titlebar__controls",
						button {
							class: "titlebar__button titlebar__button--minimize",
							onmousedown: |e| e.stop_propagation(),
							onclick: move |_| {
							    w_min.set_minimized(true);
							},
							"—"
						}
						button {
							class: "titlebar__button titlebar__button--maximize",
							onmousedown: |e| e.stop_propagation(),
							onclick: move |_| {
							    let is_max = w_max.is_maximized();
							    w_max.set_maximized(!is_max);
							},
							"□"
						}
						button {
							class: "titlebar__button titlebar__button--close",
							onmousedown: |e| e.stop_propagation(),
							onclick: move |_| {
							    w_close.close();
							},
							"×"
						}
					}
				}
			}
		}
}
