use dioxus::prelude::*;
use crate::components::navigation::Sidebar;

#[component]
pub fn HomeView() -> Element {
    rsx! {
			div { class: "main-content__lists" }
			div { class: "main-content__info-panel" }
		}
}
