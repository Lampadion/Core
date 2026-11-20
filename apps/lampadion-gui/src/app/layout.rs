use dioxus::prelude::*;
use crate::views::HomeView;
use crate::components::common::Titlebar;
use crate::components::navigation::Sidebar;

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
