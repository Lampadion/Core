use dioxus::prelude::*;
use super::views::HomeView;
use super::components::navigation::Sidebar;
use crate::shared::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
	rsx! {
		div { id: "app",
			div { class: "window-container",
				Titlebar {}
				main { class: "main-content",
					div { class: "main-content__left-island", Sidebar {} }
					div { class: "main-content__right-island", HomeView {} }
				}
			}
		}
	}
}
