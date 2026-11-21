use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::shared::icons::{TAB_COMPOSE, TAB_HOTKEY, TAB_SCRIPT};

#[component]
pub fn Sidebar() -> Element {
	html! {
		<aside class="sidebar-wrapper">
			<nav class="sidebar">
				<SidebarItem icon={TAB_HOTKEY} />
				<hr />
				<SidebarItem icon={TAB_HOTKEY} />
				<SidebarItem icon={TAB_COMPOSE} />
				<SidebarItem icon={TAB_SCRIPT} />
				<SidebarItem icon={TAB_HOTKEY} />
				<SidebarItem icon={TAB_HOTKEY} />
				<hr />
				<SidebarItem icon={TAB_HOTKEY} />
				<hr />
				<SidebarItem icon={TAB_HOTKEY} />
				<SidebarItem icon={TAB_HOTKEY} />
				<SidebarItem icon={TAB_HOTKEY} />
			</nav>
		</aside>
	}
}

#[component]
pub fn SidebarItem(icon: String) -> Element {
	html! {
		<a class="sidebar-item" href="#">
			<div class="sidebar-icon" dangerous_inner_html="{icon}"></div>
		</a>
	}
}
