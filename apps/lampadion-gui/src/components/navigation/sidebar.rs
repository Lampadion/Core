use dioxus::prelude::*;

const TAB_ICON_HOTKEY: &str = include_str!("../../../assets/graphics/icons/ui/tab-hotkey.svg");
const TAB_ICON_COMPOSE: &str = include_str!("../../../assets/graphics/icons/ui/tab-compose.svg");
const TAB_ICON_SCRIPT: &str = include_str!("../../../assets/graphics/icons/ui/tab-script.svg");

#[component]
pub fn Sidebar() -> Element {
	rsx! {
		aside { class: "sidebar-wrapper",
			nav { class: "sidebar",
				SidebarItem { icon: TAB_ICON_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_ICON_HOTKEY }
				SidebarItem { icon: TAB_ICON_COMPOSE }
				SidebarItem { icon: TAB_ICON_SCRIPT }
				SidebarItem { icon: TAB_ICON_HOTKEY }
				SidebarItem { icon: TAB_ICON_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_ICON_HOTKEY }
				hr {}
				SidebarItem { icon: TAB_ICON_HOTKEY }
				SidebarItem { icon: TAB_ICON_HOTKEY }
				SidebarItem { icon: TAB_ICON_HOTKEY }
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
