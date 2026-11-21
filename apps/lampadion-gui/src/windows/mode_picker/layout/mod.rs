use dioxus::prelude::*;

use crate::shared::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
	rsx! {
		div { id: "app",
			div { class: "window-container",
				Titlebar {}
				main { class: "main-content", "Селектор режима" }
			}
		}
	}
}
