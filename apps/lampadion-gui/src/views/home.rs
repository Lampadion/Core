use dioxus::prelude::*;
use crate::components::navigation::Sidebar;

#[component]
pub fn HomeView() -> Element {
    rsx! {
			div {
				Sidebar {}
				main {
					h1 { "Самое крутое приложение на свете" }
				}
			}
		}
}
