use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::icon::Icon;
use crate::tabs::{TabEntry, TabRegistry};
use lampadion_model::gui::{IconRef, TabId};

#[component]
pub fn Sidebar() -> Element {
	let reg = use_context::<TabRegistry>();
	let entries = reg.entries();

	html! {
		<aside class="sidebar-wrapper">
			<nav class="sidebar">
				{entries.into_iter().enumerate().map(|(i, entry)| match entry {
					TabEntry::Item(spec) => rsx! {
						SidebarItem {
							key: "{spec.decl.id.as_str()}",
							id: spec.decl.id.clone(),
							icon: spec.decl.icon.clone(),
							title: spec.decl.title.0.to_string(),
						}
					},
					TabEntry::Divider => rsx! { hr { key: "divider-{i}" } },
				})}
			</nav>
		</aside>
	}
}

#[component]
pub fn SidebarItem(id: TabId, icon: IconRef, title: String) -> Element {
	let mut active = use_context::<Signal<TabId>>();
	let is_active = *active.read() == id;

	rsx! {
		a {
			class: if is_active { "sidebar-item is-active" } else { "sidebar-item" },
			href: "#",
			title: "{title}",
			"data-tab": "{id.as_str()}",
			onclick: {
			    let id = id.clone();
			    move |e| {
			        e.prevent_default();
			        active.set(id.clone());
			    }
			},
			div { class: "sidebar-icon",
				Icon { icon }
			}
		}
	}
}
