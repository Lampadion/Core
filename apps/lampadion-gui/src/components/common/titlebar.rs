use dioxus::prelude::*;
use dioxus::desktop::use_window;

const APP_LOGO: Asset = asset!("/assets/images/lampadion.svg");

#[component]
pub fn Titlebar() -> Element {
    let window = use_window();

    rsx! {
			div {
				class: "titlebar",
				onmousedown: move |_| {
				    window.drag();
				},

				img {
					class: "titlebar__logo",
					src: APP_LOGO,
					alt: "Lampadion Logo",
					width: "24",
					height: "24",
				}
				div { class: "titlebar__title", "Lampadion" }
			}
		}
}
