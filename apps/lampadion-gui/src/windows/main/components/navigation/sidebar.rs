use dioxus::prelude::*;

use crate::shared::icons::{TAB_HOTKEY, TAB_COMPOSE, TAB_SCRIPT};

#[component]
pub fn Sidebar() -> Element {
	rsx! {
		aside { class: "sidebar-wrapper",
			nav { class: "sidebar",
				SidebarItem { icon: TAB_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_HOTKEY }
				SidebarItem { icon: TAB_COMPOSE }
				SidebarItem { icon: TAB_SCRIPT }
				SidebarItem { icon: TAB_HOTKEY }
				SidebarItem { icon: TAB_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_HOTKEY }
				SidebarItem { icon: TAB_HOTKEY }
				SidebarItem { icon: TAB_HOTKEY }
			}
		}
	}
}

#[component]
pub fn SidebarItem(icon: String) -> Element {
	rsx! {
		a { class: "sidebar-item", href: "#",
			div { class: "sidebar-icon", dangerous_inner_html: "{icon}" }
		}
	}
}
