use dioxus::prelude::*;
use crate::views::HomeView;
use crate::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
			div { id: "app",
				div { class: "window-container",
					Titlebar {}
					main { class: "main-content",
						div { class: "main-content__left-island",
							nav { class: "main-content__nav" }
						}
						div { class: "main-content__right-island", HomeView {} }
					}
				}
			}
		}
}
