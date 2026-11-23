use dioxus::prelude::*;
use dioxus_html_macro::html;

use lampadion_assets::icons::{TabCompose, TabHotkey, TabScript};

enum NavEntry {
	Item(fn() -> Element),
	Divider,
}

#[component]
pub fn Sidebar() -> Element {
	use NavEntry::{Divider, Item};

	let entries = [
		Item(TabHotkey),
		Divider,
		Item(TabHotkey),
		Item(TabCompose),
		Item(TabScript),
		Item(TabHotkey),
		Item(TabHotkey),
		Item(TabHotkey),
		Divider,
		Item(TabHotkey),
		Divider,
		Item(TabHotkey),
		Item(TabHotkey),
		Item(TabHotkey),
	];

	html! {
		<aside class="sidebar-wrapper">
			<nav class="sidebar">
				{entries.into_iter().map(|entry| match entry {
					Item(icon) => html! { <SidebarItem>{icon()}</SidebarItem> },
					Divider => html! { <hr /> },
				})}
			</nav>
		</aside>
	}
}

#[component]
pub fn SidebarItem(children: Element) -> Element {
	html! {
		<a class="sidebar-item" href="#">
			<div class="sidebar-icon">
				{children}
			</div>
		</a>
	}
}
