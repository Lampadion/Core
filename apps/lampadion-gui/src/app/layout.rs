use dioxus::prelude::*;
use crate::views::HomeView;
use crate::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
			Titlebar {}
			div { id: "app", HomeView {} }
		}
}
